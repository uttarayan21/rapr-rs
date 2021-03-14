(function() {var implementors = {};
implementors["futures_sink"] = [];
implementors["tokio_util"] = [{"text":"impl&lt;T, I, U&gt; Sink&lt;I&gt; for Framed&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncWrite,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: Encoder&lt;I&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;U::Error: From&lt;Error&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, I, D&gt; Sink&lt;I&gt; for FramedRead&lt;T, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sink&lt;I&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, I, E&gt; Sink&lt;I&gt; for FramedWrite&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsyncWrite,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Encoder&lt;I&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E::Error: From&lt;Error&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Send + 'static&gt; Sink&lt;T&gt; for PollSender&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()