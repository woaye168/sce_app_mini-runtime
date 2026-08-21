# 组装 mini-runtime 运行时载荷（B 模式脱机调试）
# 用法：powershell -File examples/assemble_runtime.ps1 [-ProjectRoot <项目路径>] [-OutDir <目标>] [-ApiVersion 13]
# 路径推导（同 sce_app_editor-patch 的 locate 链）：项目 script/tsconfig.json 的 typeRoots
# 含 <编辑器根>/update/editor-pd.spark.xd.com/res/_m/... —— 由此推编辑器 update 目录与 Res 目录。
# tester（星火对战平台）目录探测顺序：参数 > 环境变量 MINI_RUNTIME_TESTER_WIN > D:/sce_pc_tester/*/Win 扫描。
# 编辑器/tester 升级后：重跑本脚本即完成载荷升级（版本号自动按 api_pak_version.json 注册表选取）。

param(
    [string]$OutDir = (Join-Path $PSScriptRoot "..\runtime"),
    [int]$ApiVersion = 13,
    [string]$ProjectRoot = "",
    [string]$TesterWin = "",
    [string]$EditorUpdate = "",
    [string]$EditorRes = "",
    [string]$EnvDomain = "editor-pd.spark.xd.com"
)

$ErrorActionPreference = "Stop"

# ---------- 路径推导 ----------
function Derive-EditorPaths($project) {
    $tsconfig = "$project/script/tsconfig.json"
    if (-not (Test-Path $tsconfig)) { throw "项目缺 script/tsconfig.json（无法推导编辑器路径）: $tsconfig" }
    $j = Get-Content $tsconfig -Raw -Encoding utf8 | ConvertFrom-Json
    foreach ($root in $j.compilerOptions.typeRoots) {
        if ($root -match '^(.+?/update/[^/]+)/res/_m/') {
            $update = $Matches[1]
            $editorRoot = Split-Path (Split-Path $update -Parent) -Parent
            return @{ Update = $update; Res = "$editorRoot/Res" }
        }
    }
    throw "tsconfig typeRoots 里找不到 .../update/<env>/res/_m/ 形式的路径"
}

if (-not $EditorUpdate) {
    if (-not $ProjectRoot) { throw "请用 -ProjectRoot <项目路径> 推导官方目录（或直接传 -EditorUpdate/-EditorRes/-TesterWin）" }
    $derived = Derive-EditorPaths $ProjectRoot
    $EditorUpdate = $derived.Update
    if (-not $EditorRes) { $EditorRes = $derived.Res }
}
if (-not $TesterWin) { $TesterWin = $env:MINI_RUNTIME_TESTER_WIN }
if (-not $TesterWin) {
    $TesterWin = (Get-ChildItem "D:/sce_pc_tester/*/Win" -Directory -ErrorAction SilentlyContinue |
                  Sort-Object LastWriteTime -Descending | Select-Object -First 1).FullName
}
if (-not $TesterWin) { throw "找不到 tester（星火对战平台）Win 目录；请传 -TesterWin 或设环境变量 MINI_RUNTIME_TESTER_WIN" }

# 统一正斜杠（引擎/日志风格一致）
$OutDir = $OutDir -replace '\\', '/'
$EditorUpdate = $EditorUpdate -replace '\\', '/'
$EditorRes = $EditorRes -replace '\\', '/'
$TesterWin = $TesterWin -replace '\\', '/'

Write-Host "tester : $TesterWin"
Write-Host "editor : $EditorUpdate"
Write-Host "out    : $OutDir"

$dstUpdate = "$OutDir/Update/$EnvDomain"
$dstRes = "$dstUpdate/Res"

function Copy-Dir($src, $dst) {
    if (-not (Test-Path $src)) { Write-Host "[skip] 不存在: $src"; return }
    robocopy $src $dst /E /NFL /NDL /NJH /NJS /np | Out-Null
    Write-Host "[ok] $src -> $dst"
}

# ① 引擎二进制（tester 的 scegame + 必需 dll）
New-Item -ItemType Directory -Force $OutDir | Out-Null
Copy-Item "$TesterWin/scegame" "$OutDir/scegame.exe" -Force
foreach ($dll in @("lua54.dll","shaderc.dll","themis_x64.dll","sdk.dll","lite.dll",
                   "gmesdk.dll","libgmecrypto-1_1-x64.dll","libgmefaad2.dll","libgmefdkaac.dll",
                   "libgmelamemp3.dll","libgmeogg.dll","libgmesoundtouch.dll","libgmessl-1_1-x64.dll",
                   "msvcp140.dll","msvcp140_1.dll","msvcp140_2.dll",
                   "vcruntime140.dll","vcruntime140_1.dll","ucrtbase.dll","d3dcompiler_47.dll")) {
    $s = "$TesterWin/$dll"
    if (Test-Path $s) { Copy-Item $s $OutDir -Force } else { Write-Host "[warn] 缺 dll: $dll" }
}
Write-Host "[ok] 引擎二进制"

# ② 版本注册表（库版本解析的根；缺失会导致 @lib 模块找不到）
New-Item -ItemType Directory -Force $dstUpdate | Out-Null
New-Item -ItemType Directory -Force $dstRes | Out-Null
foreach ($f in @("api_pak_version.json","map_pak_version.json","VERSION.JSON")) {
    Copy-Item "$EditorUpdate/$f" $dstUpdate -Force
}
Write-Host "[ok] 版本注册表"

# ③ 核心散包（Update/<env>/Res/ 下）+ 游戏字体族（ui/font/regular——UI 文字必需）
foreach ($d in @("script","client_base","startup","appui","engineres","fonts","uistyle","lite","refconfig","shadercache_windows_ui")) {
    Copy-Dir "$EditorUpdate/res/$d" "$dstRes/$d"
}
Copy-Dir "$EditorUpdate/res/ui/font/regular" "$dstRes/ui/font/regular"

# ④ _m 高版本包：版本号按 api_pak_version.json[<api>] 自动选取
$apiPak = Get-Content "$EditorUpdate/api_pak_version.json" -Raw -Encoding utf8 | ConvertFrom-Json
$apiEntry = $apiPak.PSObject.Properties[[string]$ApiVersion].Value
function PakVer($name) {
    if ($apiEntry -and $apiEntry.PSObject.Properties[$name]) { return [string]$apiEntry.PSObject.Properties[$name].Value }
    return $null
}
foreach ($pkg in @("script","appui","gameui","shadercache_editor_dxbc","shadercache_editor_dxbc_extra")) {
    $ver = PakVer $pkg
    if (-not $ver) {
        # 注册表没有则取本地最大版本
        $base = "$EditorUpdate/res/_m/$pkg"
        $ver = (Get-ChildItem $base -Directory -ErrorAction SilentlyContinue |
                Where-Object { $_.Name -match '^\d+$' } | Sort-Object {[int]$_.Name} -Descending |
                Select-Object -First 1).Name
    }
    if ($ver) { Copy-Dir "$EditorUpdate/res/_m/$pkg/$ver" "$dstRes/_m/$pkg/$ver" }
    else { Write-Host "[warn] 包无版本: $pkg" }
}

# ⑤ 项目依赖库（_m/maps）——版本按 api_pak_version.json[<api>] 注册表选取（与编辑器一致），无注册取本地最大
foreach ($lib in @("global_default","defaultui","default_units_ts",
                   "script_libs/lib_control","script_libs/lib_game_options","script_libs/lib_common_sounds",
                   "script_libs/smallcard_get_items","script_libs/smallcard_inventory","script_libs/smallcard_mail",
                   "ai_templates/lib_common_ai")) {
    $base = "$EditorUpdate/res/_m/maps/$lib"
    $short = Split-Path $lib -Leaf
    $ver = PakVer $short
    if (-not $ver) {
        $ver = (Get-ChildItem $base -Directory -ErrorAction SilentlyContinue |
                Where-Object { $_.Name -match '^\d+$' } | Sort-Object {[int]$_.Name} -Descending |
                Select-Object -First 1).Name
    }
    if ($ver) { Copy-Dir "$base/$ver" "$dstRes/_m/maps/$lib/$ver" }
    else { Write-Host "[warn] 库不存在: $lib" }
}

# ⑥ 基础资源（编辑器版 characters/effect——不是 tester 的大厅资源！）
Copy-Dir "$EditorRes/characters" "$OutDir/Res/characters"
Copy-Dir "$EditorRes/effect" "$OutDir/Res/effect"

# ⑦ 空目录骨架
foreach ($d in @("ResCache","User","User/debug")) {
    New-Item -ItemType Directory -Force "$OutDir/$d" | Out-Null
}

$size = (Get-ChildItem $OutDir -Recurse -File | Measure-Object Length -Sum).Sum / 1MB
Write-Host ("`n===== 载荷组装完成: {0}（{1:N0} MB）=====" -f $OutDir, $size)
Write-Host "升级方法：编辑器/tester 升级后重跑本脚本即可（版本号自动跟随 api_pak_version.json）。"
