# 探测 updater update-info 的正确调用方式（带签名头）
import json, sys, time, hashlib, random, urllib.request

cred_path = r'd:\sce_online\Res\maps\sce_app_mini-runtime\target\debug\sce_app_mini-runtime.credentials.json'
store = json.load(open(cred_path, encoding='utf-8'))
cred = store['items'][store['active_label']]['info']
login_token = cred['login_token']
secret = cred['login_token_secret']

body = b'{"version":2,"default_part":1,"sample":0,"suffix":"client","api_version":0,"list":"test_res002;test_res002;test_res002;global_default"}'

content_md5 = hashlib.md5(body).hexdigest()
noise = str(random.randint(1000000, 9999999))
time_str = str(int(time.time()))
pre = f"{noise}\n{time_str}\n{content_md5}\n{login_token}\n{secret}"
sign = hashlib.md5(pre.encode()).hexdigest()

req = urllib.request.Request(
    "https://updater-pd.tapsce.cn/api/map/update-info",
    data=body,
    headers={
        "Content-Type": "application/json",
        "noise": noise, "time_str": time_str, "content_md5": content_md5,
        "token": login_token, "sign": sign,
    },
    method="POST",
)
proxy = urllib.request.ProxyHandler({"https": "http://127.0.0.1:7897", "http": "http://127.0.0.1:7897"})
opener = urllib.request.build_opener(proxy)
try:
    resp = opener.open(req, timeout=30)
    text = resp.read().decode('utf-8', 'replace')
    print("status", resp.status)
    print(text[:3000])
except Exception as e:
    print("ERR", e)
    if hasattr(e, 'read'):
        print(e.read().decode('utf-8', 'replace')[:500])
