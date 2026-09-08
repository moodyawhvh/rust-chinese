> 🌐 本文档由 [rust-lang/rust](https://github.com/rust-lang/rust) 翻译,英文原版见原项目。

% 页面未找到

<!-- Completely hide the TOC and the section numbers -->
<style type="text/css">
#rustdoc-toc { display: none; }
.header-section-number { display: none; }
li {list-style-type: none; }
#search-input {
    width: calc(100% - 100px);
}
#search-but {
    cursor: pointer;
}
#search-but, #search-input {
    padding: 4px;
    border: 1px solid #ccc;
    border-radius: 3px;
    outline: none;
    font-size: 0.7em;
    background-color: #fff;
}
#search-but:hover, #search-input:focus {
    border-color: #55a9ff;
}
#search-from {
    border: none;
    padding: 0;
    font-size: 0.7em;
}
</style>

看起来你走错路了。

下面这些也许能帮到你:

# 搜索

<div>
  <form id="search-form" action="https://duckduckgo.com/">
    <input id="search-input" type="search" name="q"></input>
    <input type="submit" value="搜索" id="search-but">
    <!--
      Don't show the options by default,
      since "From the Standard Library" doesn't work without JavaScript
    -->
    <fieldset id="search-from" style="display:none">
      <label><input name="from" value="library" type="radio"> 标准库内搜索</label>
      <label><input name="from" value="duckduckgo" type="radio" checked> 用 DuckDuckGo 搜索</label>
    </fieldset>
  </form>
</div>

# 参考资源

 * [Rust 官方网站](https://www.rust-lang.org)
 * [Rust 参考手册](https://doc.rust-lang.org/reference/index.html)

# 文档

[标准库文档](https://doc.rust-lang.org/std/)

<script>
function get_url_fragments() {
    var last = document.URL.split("/").pop();
    var tokens = last.split(".");
    var op = [];
    for (var i=0; i < tokens.length; i++) {
        var t = tokens[i];
        if (t == 'html' || t.indexOf("#") != -1) {
            // no html or anchors
        } else {
            op.push(t);
        }
    }
    return op;
}

function on_submit(event) {
    var form = event.target;
    var q = form['q'].value;

    event.preventDefault();

    if (form['from'].value === 'duckduckgo') {
        document.location.href = form.action + '?q=' + encodeURIComponent(q + ' site:doc.rust-lang.org');
    } else if (form['from'].value === 'library') {
        document.location.href = '/std/index.html?search=' + encodeURIComponent(q);
    }
}

function populate_search() {
    var form = document.getElementById('search-form');
    form.addEventListener('submit', on_submit);
    document.getElementById('search-from').style.display = '';

    form['from'].value = 'library';

    var op = get_url_fragments();
    document.getElementById('search-input').value = op.join(' ');
}
populate_search();
</script>
