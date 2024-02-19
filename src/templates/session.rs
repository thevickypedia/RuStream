/// Get the HTML content to render the session expired/invalid page.
///
/// # See Also
///
/// - This page is served as a response for all the content delivery entry points,
/// when the user's `session_token` is invalid or expired.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
    <title>Rustic video streaming</title>
    <meta property="og:type" content="VideoStreaming">
    <meta name="keywords" content="Python, streaming, fastapi, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <style>
        img {
            display: block;
            margin-left: auto;
            margin-right: auto;
        }

        :is(h1, h2, h3, h4, h5, h6) {
            text-align: center;
            color: #F0F0F0;
            font-family: 'Courier New', sans-serif;
        }

        button {
            width: 100px;
            margin: 0 auto;
            display: block;
        }

        body {
            background-color: #151515;
        }
    </style>
    <noscript>
        <style>
            body {
                width: 100%;
                height: 100%;
                overflow: hidden;
            }
        </style>
        <div style="position: fixed; text-align:center; height: 100%; width: 100%; background-color: #151515">
            <h2 style="margin-top:5%">This page requires JavaScript
                to be enabled.
                <br><br>
                Please refer <a href="https://www.enable-javascript.com/">enable-javascript</a> for how to.
            </h2>
            <form>
                <button type="submit" onClick="<meta httpEquiv='refresh' content='0'>">RETRY</button>
            </form>
        </div>
    </noscript>
</head>
<body>
<h2 style="margin-top:5%">{{ reason }}</h2>
<h3>Authentication doesn't last forever ¯\_(ツ)_/¯ </h3>
<p>
    <img src="https://raw.githubusercontent.com/thevickypedia/open-source/main/images/session.gif"
        onerror="this.src='https://vigneshrao.com/open-source/images/session.gif'"
        width="200" height="200" alt="Image" class="center">
</p>
<button style="text-align:center" onClick="window.location.href = '/';">LOGIN</button>
<br>
<button style="text-align:center" onClick="alert('Forgot Password?\n\nRelax and try to remember your password.');">HELP
</button>
<h4>Click <a href="https://vigneshrao.com/contact">HERE</a> to reach out.</h4>
</body>
</html>
"###.to_string()
}
