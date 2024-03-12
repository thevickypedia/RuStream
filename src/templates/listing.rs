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
    <!-- Favicon.ico and Apple Touch Icon -->
    <link rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.ico">
    <link rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.png">
    <!-- Font Awesome icons -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/fontawesome.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/solid.css">
    <!-- CSS and JS for night mode -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/2.2.2/jquery.min.js"></script>
    <script type="text/javascript" src="https://thevickypedia.github.io/open-source/nightmode/night.js" defer></script>
    <link rel="stylesheet" type="text/css" href="https://thevickypedia.github.io/open-source/nightmode/night.css">
    <!-- Button CSS -->
    <style>
        /* Google fonts with a backup alternative */
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@400;500;700&display=swap');
        * {
            font-family: 'Ubuntu', 'PT Serif', sans-serif;
        }
        body {
            margin-left: 1%;  /* 1% away from left corner */
            padding: 0.5%  /* 0.5% away from any surrounding elements */
        }
        .upload {
            position: absolute;
            top: 3.8%;
            right: 313px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
        .home {
            position: absolute;
            top: 3.8%;
            right: 217px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
        .back {
            position: absolute;
            top: 3.8%;
            right: 132px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }
    </style>
    <style>
        .dropbtn {
            position: absolute;
            top: 3.8%;
            right: 30px;
            padding: 10px 24px;
            font-size: 16px;
            border: none;
            cursor: pointer;
        }
        .dropdown {
            position: absolute;
            top: 3.8%;
            right: 30px;
            padding: 10px 24px;
            display: inline-block;
        }
        .dropdown-content {
            display: none;
            position: absolute;
            top: 40px;  /* Distance from the user icon button */
            right: 30px;
            width: 160px;
            min-width: auto;
            box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);  /* Basically, black with 20% opacity */
            z-index: 1;
        }
        .dropdown-content a {
            padding: 12px 16px;
            text-decoration: none;
            display: block;
        }
        .dropdown:hover .dropdown-content {display: block;}
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
    <button class="upload" onclick="upload()"><i class="fa-solid fa-cloud-arrow-up"></i> Upload</button>
    <button class="home" onclick="goHome()"><i class="fa fa-home"></i> Home</button>
    <button class="back" onclick="goBack()"><i class="fa fa-backward"></i> Back</button>
    <div class="dropdown">
        <button class="dropbtn"><i class="fa fa-user"></i></button>
        <div class="dropdown-content">
            <a onclick="goSecure()" style="cursor: pointer;"><i class="fa-solid fa-user-lock"></i> {{ user }}</a>
            <a onclick="logOut()" style="cursor: pointer"><i class="fa fa-sign-out"></i> logout</a>
        </div>
    </div>
    <br><br><br><br>
    {% if custom_title %}
        <h1>{{ custom_title }}</h1>
    {% else %}
        <h1>RuStream - Self-hosted Streaming Engine</h1>
    {% endif %}
    <hr>
    {% if dir_name or files or directories or secured_directories %}
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
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
        {% if secured_directories %}
            <h3>Secured Directory</h3>
            {% for directory in secured_directories %}
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
    {% else %}
        <h3 style="text-align: center">No content was rendered by the server</h3>
    {% endif %}
    <hr>
    <script>
        function goHome() {
            window.location.href = window.location.origin + "/home";
        }
        function goSecure() {
            window.location.href = window.location.origin + '/{{ secure_index }}';
        }
        function logOut() {
            window.location.href = window.location.origin + "/logout";
        }
        function upload() {
            window.location.href = window.location.origin + "/upload";
        }
        function goBack() {
            window.history.back();
        }
    </script>
</body>
</html>
"###.to_string()
}
