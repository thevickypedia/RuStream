/// Get the HTML content to render the home/listing page.
///
/// # See Also
///
/// - This page is served as a response for the `/home` entry point.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<!--suppress JSUnresolvedLibraryURL -->
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>RuStream - Self-hosted Streaming Engine</title>
    <meta property="og:type" content="MediaStreaming">
    <meta name="keywords" content="Rust, streaming, actix, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <!-- Disables 404 for favicon.ico which is a logo on top of the webpage tab -->
    <link rel="shortcut icon" href="#">
    <!-- Font Awesome icons -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/fontawesome.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/solid.css">
    <!-- CSS and JS for night mode -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/2.2.2/jquery.min.js"></script>
    <script type="text/javascript" src="https://thevickypedia.github.io/open-source/nightmode/night.js" defer></script>
    <link rel="stylesheet" type="text/css" href="https://thevickypedia.github.io/open-source/nightmode/night.css">
    <!-- Button CSS -->
    <style>
        body {
            margin-left: 1%;  /* 1% away from left corner */
            padding: 0.5%  /* 0.5% away from any surrounding elements */
        }
        .home {
            position: absolute;
            top: 3.8%;
            right: 218px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
        .back {
            position: absolute;
            top: 3.8%;
            right: 130px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
        .logout {
            position: absolute;
            top: 3.8%;
            right: 30px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
    </style>
    <!-- Title list CSS -->
    <style>
        a:hover, a:active { font-size: 120%; opacity: 0.7; }
        a:link { color: blue; }
        a:visited { color: blue; }
        ol {
            list-style: none;
            counter-reset: list-counter;
        }
        li {
            margin: 1rem;
            list-style-type: none; /* Hide default marker */
        }
        li::before {
            background: #4169E1;
            width: 2rem;
            height: 2rem;
            border-radius: 50%;
            display: inline-block;
            line-height: 2rem;
            color: white;
            text-align: center;
            margin-right: 0.5rem;
        }
        body {
            font-family: 'PT Serif', serif;
        }
    </style>
</head>
<noscript>
    <style>
        body {
            width: 100%;
            height: 100%;
            overflow: hidden;
        }
    </style>
    <div style="position: fixed; text-align:center; height: 100%; width: 100%; background-color: #151515;">
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
<body translate="no">
    <div class="toggler fa fa-moon-o"></div>
    <br><br>
    {% if custom_title %}
        <h1>{{ custom_title }}</h1>
    {% else %}
        <h1>RuStream - Self-hosted Streaming Engine</h1>
    {% endif %}
    <hr>
    <button class="home" onclick="goHome()"><i class="fa fa-home"></i> Home</button>
    <button class="back" onclick="goBack()"><i class="fa fa-backward"></i> Back</button>
    <button class="logout" onclick="logOut()"><i class="fa fa-sign-out"></i> Logout</button>
    {% if dir_name or files or directories %}
        <!-- Display directory name if within subdir -->
        {% if dir_name %}
            <h3>{{ dir_name }}</h3>
        {% endif %}
        <!-- Display number of files and list the files -->
        {% if files %}
            <h3>Files {{ files|length }}</h3>
            {% for file in files %}
                <li><i class="{{ file.font }}"></i>&nbsp;&nbsp;<a href="{{ file.path }}">{{ file.name }}</a></li>
            {% endfor %}
        {% endif %}
        <!-- Display number of directories and list the directories -->
        {% if directories %}
            <h3>Directories {{ directories|length }}</h3>
            {% for directory in directories %}
                {% if '/' in directory.name %}
                    <li><i class="fa-solid fa-folder-tree"></i>&nbsp;&nbsp;<a href="{{ directory.path }}">{{ directory.name }}</a></li>
                {% else %}
                    <li><i class="fa fa-folder"></i>&nbsp;&nbsp;<a href="{{ directory.path }}">{{ directory.name }}</a></li>
                {% endif %}
            {% endfor %}
        {% endif %}
    {% else %}
        <h3 style="text-align: center">No content was rendered by the server</h3>
    {% endif %}
    <hr>
    <script>
        function logOut() {
            window.location.href = window.location.origin + "/logout";
        }
        function goHome() {
            window.location.href = window.location.origin + "/home";
        }
        function goBack() {
            window.history.back();
        }
    </script>
</body>
</html>
"###.to_string()
}
