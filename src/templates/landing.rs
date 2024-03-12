/// Get the HTML content to render the streaming/landing page.
///
/// # See Also
///
/// - This page is served as a response for the `/stream` entry point.
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
    <title>{{ media_title }}</title>
    <meta property="og:type" content="MediaStreaming">
    <meta name="keywords" content="Rust, streaming, actix, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <!-- CSS and JS for video-js plugin -->
    <!-- If you'd like to support IE8 (for Video.js versions prior to v7) -->
    <!-- <script src="https://thevickypedia.github.io/open-source/videojs/videojs-ie8.js"></script> -->
    <link href="https://thevickypedia.github.io/open-source/videojs/video.css" rel="stylesheet"/>
    <script src="https://thevickypedia.github.io/open-source/videojs/video.js" defer></script>
    <!-- Favicon.ico and Apple Touch Icon -->
    <link rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.ico">
    <link rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.png">
    <!-- Font Awesome icons -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/fontawesome.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/solid.css">
    <!-- Button CSS -->
    <style>
        /* Google fonts with a backup alternative */
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@400;500;700&display=swap');
        * {
            font-family: 'Ubuntu', 'PT Serif', sans-serif;
        }
        .iter {
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
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
        body {
            background-color: #151515;
        }
        title, h1, h2, h3, h4, h5, h6, p, a {
            color: #f0f0f0;
        }
        button {
            background: transparent !important;
            color: #f0f0f0;
        }
        button:hover {
            background: transparent !important;
            opacity: 0.6;
            transition: 0.5s;
        }
    </style>
    <!-- Container, title and body CSS -->
    <style>
        h1 {
            text-align: center;
        }
    </style>
    <!-- Size of the container and the player -->
    <style>
        body {
            margin: 0; /* Remove default margin */
            padding: 0; /* Remove default padding */
            box-sizing: border-box; /* Include padding and border in element's total width and height */
        }
        #content-container {
            position: relative;
            width: 70%;
            max-width: 100%; /* Set a maximum width to prevent overflow */
            height: 75vh; /* Set height to 75% of the viewport height */
            margin: 0 auto; /* Center the container horizontally */
        }
        #nav-container {
            position: relative;
            width: 70%;
            margin: 0 auto; /* Center the container horizontally */
        }
        #image-source {
            max-width: 100%;
            height: 75vh;
            margin: 0 auto; /* Center the container horizontally */
            display: flex;
            justify-content: center;
            align-items: center; /* Center the container vertically */
            cursor: pointer; /* Add a pointer cursor to indicate it's clickable */
            overflow: hidden; /* Avoid vertical overflow */
        }
        #video-player {
            position: relative;
            height: 100%;
            width: 100%;
            display: block;
        }
        @media (max-width: 768px) {
            #image-source {
                height: auto;
                width: 90%;
            }
            /* video-js plugin defaults to 120% on mobile phones, so use the same */
            #content-container {
                height: auto;
                width: 120%;
            }
            #video-player {
                height: auto;
                width: 120%;
            }
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
</head>
<body>
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
    <br><br><br>
    <h1>{{ media_title }}</h1>
    {% if render_image %}
        <img id="image-source" src="" onclick="fullScreen()">
    {% else %}
        <div id="content-container">
            <video id="video-player"
                   class="video-js"
                   preload="auto"
                   controls muted="muted"
                   style="position: relative; margin-left: auto; margin-right: auto; display: block"
                   data-setup='{
                 "playbackRates": [1, 1.5, 2, 5],
                 "controlBar": {
                   "skipButtons": {
                     "backward": 10,
                     "forward": 10
                   }
                 }
               }'>
                <source id="video-source" type="video/mp4" src=""/>
                <track id="subtitles" kind="subtitles" src="" srclang="en"/>
                <p class="vjs-no-js">
                    To view this video please enable JavaScript, and consider upgrading to a
                    web browser that
                    <a href="https://videojs.com/html5-video-support/" target="_blank">supports HTML5 video</a>
                </p>
            </video>
        </div>
    {% endif %}
    <div id="nav-container">
        {% if previous %}
            <button class="iter" style="float: left" onclick="window.location='{{ previous }}'" title="{{ previous }}">
                <i class="fa fa-backward"></i> Previous
            </button>
        {% endif %}
        {% if next %}
            <button class="iter" style="float: right" onclick="window.location='{{ next }}'" title="{{ next }}">
                Next <i class="fa fa-forward"></i>
            </button>
        {% endif %}
        <br><br>
    </div>
    <script>
        let origin = window.location.origin; // Get the current origin using JavaScript
        let path = "{{ path }}";
        {% if render_image %}
            // Construct the source URL for the image by combining origin and path
            let imageSource = origin + path;

            // Set the image source URL for the image-source element
            let imageElement = document.getElementById("image-source");
            imageElement.setAttribute("src", imageSource);
        {% else %}
            let track = "{{ track }}";

            // Construct the source URL for video by combining origin and path
            let videoSource = origin + path;

            // Set the video source URL for the video-source element
            let videoElement = document.getElementById("video-source");
            videoElement.setAttribute("src", videoSource);

            // Set the subtitles URL for the video
            let trackElement = document.getElementById("subtitles");
            trackElement.setAttribute("src", track);

            let videoPlayer = document.getElementById("video-player");
            videoPlayer.load(); // Load the video
            // videoPlayer.play(); // Play the video
        {% endif %}
    </script>
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
    <script>
        function fullScreen() {
            var doc = window.document;
            // var docEl = doc.documentElement;  // Entire container as fullScreen
            var docEl = document.getElementById("image-source");  // ONLY image as fullScreen
            var requestFullScreen =
                docEl.requestFullscreen ||
                docEl.mozRequestFullScreen ||
                docEl.webkitRequestFullScreen ||
                docEl.msRequestFullscreen;
            var cancelFullScreen =
                doc.exitFullscreen ||
                doc.mozCancelFullScreen ||
                doc.webkitExitFullscreen ||
                doc.msExitFullscreen;
            if (
                !doc.fullscreenElement &&
                !doc.mozFullScreenElement &&
                !doc.webkitFullscreenElement &&
                !doc.msFullscreenElement
            ) {
                if (requestFullScreen === undefined) {
                    alert("Failed to render {{ media_title }} in fullScreen");
                }
                requestFullScreen.call(docEl);
            } else {
                if (cancelFullScreen === undefined) {
                    alert("Failed to cacnel fullScreen for {{ media_title }}");
                }
                cancelFullScreen.call(doc);
            }
        }
        {% if previous %}
            // Add event listener for the left arrow key
            document.addEventListener('keydown', navigateLeft);
            function navigateLeft(event) {
                if (event.key === 'ArrowLeft') {
                    // Navigate to the previous image
                    window.location='{{ previous }}';
                }
            }
        {% endif %}
        {% if next %}
            // Add event listener for the right arrow key
            document.addEventListener('keydown', navigateRight);
            function navigateRight(event) {
                if (event.key === 'ArrowRight') {
                    // Navigate to the next image
                    window.location='{{ next }}';
                }
            }
        {% endif %}
    </script>
</body>
</html>
"###.to_string()
}
