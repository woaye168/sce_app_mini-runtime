# 本地明文捕获+转发服务器：配合 scegame -http=127.0.0.1 抓 update-info 完整请求
import http.server, socketserver, json, sys, urllib.request

REAL = "https://updater-pd.tapsce.cn"
CAPTURE = r"C:\Users\woaye\AppData\Local\Temp\http_capture.txt"

class H(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        body = self.rfile.read(int(self.headers.get('Content-Length', 0)))
        with open(CAPTURE, 'a', encoding='utf-8') as f:
            f.write(f"=== {self.command} {self.path}\n")
            for k, v in self.headers.items():
                f.write(f"{k}: {v}\n")
            f.write(f"BODY({len(body)}): {body.decode('utf-8','replace')}\n\n")
        # 转发真实服务（经代理）
        try:
            proxy = urllib.request.ProxyHandler({"https": "http://127.0.0.1:7897"})
            opener = urllib.request.build_opener(proxy)
            req = urllib.request.Request(REAL + self.path, data=body,
                                         headers={"Content-Type": self.headers.get("Content-Type", "application/json")},
                                         method="POST")
            resp = opener.open(req, timeout=30)
            data = resp.read()
            self.send_response(resp.status)
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
            with open(CAPTURE, 'a', encoding='utf-8') as f:
                f.write(f"RESP {resp.status}: {data.decode('utf-8','replace')[:2000]}\n\n")
        except Exception as e:
            msg = str(e).encode()
            self.send_response(502)
            self.send_header("Content-Length", str(len(msg)))
            self.end_headers()
            self.wfile.write(msg)

    def do_GET(self):
        self.do_POST()

    def log_message(self, *a):
        pass

with socketserver.TCPServer(("127.0.0.1", 9002), H) as srv:
    print("listening 9002", flush=True)
    srv.serve_forever()
