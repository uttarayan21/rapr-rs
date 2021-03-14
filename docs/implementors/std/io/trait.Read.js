(function() {var implementors = {};
implementors["base64"] = [{"text":"impl&lt;'a, R:&nbsp;Read&gt; Read for DecoderReader&lt;'a, R&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl&lt;B:&nbsp;Buf + Sized&gt; Read for Reader&lt;B&gt;","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl Read for Receiver","synthetic":false,"types":[]},{"text":"impl Read for &amp;Receiver","synthetic":false,"types":[]},{"text":"impl Read for TcpStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a TcpStream","synthetic":false,"types":[]},{"text":"impl Read for UnixStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a UnixStream","synthetic":false,"types":[]}];
implementors["native_tls"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Read for TlsStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Read for SslStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["socket2"] = [{"text":"impl Read for Socket","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a Socket","synthetic":false,"types":[]}];
implementors["tokio_native_tls"] = [{"text":"impl&lt;S&gt; Read for AllowStd&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: AsyncRead + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()