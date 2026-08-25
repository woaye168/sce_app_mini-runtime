-- W1 探针页面：canvas2d 棋盘 + 每秒 scelua.send_string 心跳 + __fromLua 回写入口
return [[<html><body style='margin:0;background:#223'>
<canvas id='c' width='400' height='300'></canvas>
<script>
var n=0;
var x=document.getElementById('c').getContext('2d');
var luaMsg='';
function draw(){
  x.fillStyle='#224';x.fillRect(0,0,400,300);
  for(var i=0;i<8;i++){for(var j=0;j<6;j++){x.fillStyle=((i+j)%2==0)?'#c63':'#36c';x.fillRect(i*50,j*50,48,48);}}
  x.fillStyle='#fff';x.font='20px sans-serif';x.fillText('CANVAS2D '+n,10,290);
  if(luaMsg){ x.fillStyle='#ff0'; x.font='30px sans-serif'; x.fillText(luaMsg,100,150); }
}
draw();
try{ scelua.send_string('page-loaded'); }catch(e){ try{ chrome.webview.postMessage('page-loaded-via-chrome'); }catch(e2){} }
setInterval(function(){ n++; draw(); try{ scelua.send_string('tick '+n); }catch(e){} }, 3000);
window.__fromLua = function(s){ luaMsg=s; draw(); };
</script></body></html>]]
