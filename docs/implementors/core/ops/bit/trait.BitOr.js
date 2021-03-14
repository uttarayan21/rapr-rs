(function() {var implementors = {};
implementors["hashbrown"] = [{"text":"impl&lt;T, S&gt; BitOr&lt;&amp;'_ HashSet&lt;T, S&gt;&gt; for &amp;HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;T, S1, S2&gt; BitOr&lt;&amp;'_ IndexSet&lt;T, S2&gt;&gt; for &amp;IndexSet&lt;T, S1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S1: BuildHasher + Default,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl BitOr&lt;Interest&gt; for Interest","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl BitOr&lt;CMSOptions&gt; for CMSOptions","synthetic":false,"types":[]},{"text":"impl BitOr&lt;OcspFlag&gt; for OcspFlag","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Pkcs7Flags&gt; for Pkcs7Flags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SslOptions&gt; for SslOptions","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SslMode&gt; for SslMode","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SslVerifyMode&gt; for SslVerifyMode","synthetic":false,"types":[]},{"text":"impl BitOr&lt;SslSessionCacheMode&gt; for SslSessionCacheMode","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ExtensionContext&gt; for ExtensionContext","synthetic":false,"types":[]},{"text":"impl BitOr&lt;ShutdownState&gt; for ShutdownState","synthetic":false,"types":[]},{"text":"impl BitOr&lt;X509CheckFlags&gt; for X509CheckFlags","synthetic":false,"types":[]},{"text":"impl BitOr&lt;X509VerifyFlags&gt; for X509VerifyFlags","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl BitOr&lt;Interest&gt; for Interest","synthetic":false,"types":[]},{"text":"impl BitOr&lt;Ready&gt; for Ready","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()