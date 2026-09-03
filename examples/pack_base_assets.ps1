# 重新打包 base_assets.7z（基座资产：ui/font/regular 游戏字体族 + fonts + characters + effect）
# 这些是 update-info 不分发、仅编辑器安装包携带的资产，payload sync 走
# 「本机编辑器兜底复制 / GitHub release 下载」双通道，本脚本负责从本机编辑器目录生成该包。
#
# 用法：powershell -File examples/pack_base_assets.ps1 -ProjectRoot <项目路径> [-Out <输出.7z>]
# 路径推导同 assemble_runtime.ps1：项目 script/tsconfig.json 的 typeRoots。
# 编辑器升级导致字体/资产变化后重跑本脚本，然后：
#   gh release upload <tag> examples/base_assets.7z --clobber

param(
    [string]$ProjectRoot = "",
    [string]$EditorUpdate = "",
    [string]$EditorRes = "",
    [string]$Out = (Join-Path $PSScriptRoot "base_assets.7z")
)

$ErrorActionPreference = "Stop"

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
    if (-not $ProjectRoot) { throw "请用 -ProjectRoot <项目路径> 推导官方目录（或直接传 -EditorUpdate/-EditorRes）" }
    $derived = Derive-EditorPaths $ProjectRoot
    $EditorUpdate = $derived.Update
    if (-not $EditorRes) { $EditorRes = $derived.Res }
}
$EditorUpdate = $EditorUpdate -replace '\\', '/'
$EditorRes = $EditorRes -replace '\\', '/'
$Out = $Out -replace '\\', '/'

# 包内布局（与 src/core/payload.rs sync_base_assets 的落位一一对应）：
#   ui/font/regular/... → Update/<env>/Res/ui/    fonts/... → Update/<env>/Res/fonts/
#   characters/... → Res/characters/              effect/... → Res/effect/
$stage = Join-Path $env:TEMP ("base_assets_" + [guid]::NewGuid().ToString("N"))
try {
    New-Item -ItemType Directory -Force "$stage/ui/font" | Out-Null
    foreach ($pair in @(
        @("$EditorUpdate/res/ui/font/regular", "$stage/ui/font/regular"),
        @("$EditorUpdate/res/fonts",           "$stage/fonts"),
        @("$EditorRes/characters",             "$stage/characters"),
        @("$EditorRes/effect",                 "$stage/effect")
    )) {
        $src, $dst = $pair
        if (-not (Test-Path $src)) { throw "源目录不存在: $src" }
        robocopy $src $dst /E /NFL /NDL /NJH /NJS /np | Out-Null
        # robocopy 退出码 >=8 表示失败（$ErrorActionPreference 对原生进程不生效，需显式检查）
        if ($LASTEXITCODE -ge 8) { throw "robocopy 失败($LASTEXITCODE): $src" }
        Write-Host "[ok] $src"
    }
    if (Test-Path $Out) { Remove-Item $Out -Force }
    tar -acf $Out -C $stage ui fonts characters effect
    if ($LASTEXITCODE -ne 0) { throw "tar 打包失败" }
    $mb = (Get-Item $Out).Length / 1MB
    Write-Host ("`n===== base_assets 打包完成: {0}（{1:N1} MB）=====" -f $Out, $mb)
    Write-Host "上传到 release：gh release upload <tag> $Out --clobber"
} finally {
    Remove-Item $stage -Recurse -Force -ErrorAction SilentlyContinue
}
