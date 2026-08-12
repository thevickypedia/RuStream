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
    {% if media_title %}
        <title>{{ media_title }}</title>
    {% else %}
        <title>RuStream - Self-hosted Streaming Engine - v{{ version }}</title>
    {% endif %}
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
        .navbar {
            position: relative;
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 1% 1.5%;
        }
        .navbar button {
            border: none;
            padding: 8px 14px;
            font-size: 15px;
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
        .user-actions {
            position: absolute;
            top: 3.8%;
            right: 30px;
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .user-actions button {
            border: none;
            padding: 8px 14px;
            font-size: 15px;
            cursor: pointer;
        }
    </style>
    <!-- Upload dialog CSS -->
    <style>
        .upload-overlay {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.55);
            align-items: center;
            justify-content: center;
            z-index: 998;
        }
        .upload-modal {
            background: #ffffff;
            border-radius: 12px;
            padding: 20px 24px;
            width: 92%;
            max-width: 440px;
            max-height: 85vh;
            overflow-y: auto;
            position: relative;
        }
        .upload-modal .close {
            position: absolute;
            top: 10px;
            right: 14px;
            background: transparent !important;
            border: none;
            font-size: 18px;
            cursor: pointer;
            color: #666666 !important;
        }
        .upload-modal h3 {
            margin: 0 0 4px;
            text-align: center;
            color: #000000 !important;
        }
        .upload-modal p.sub {
            margin: 0 0 14px;
            text-align: center;
            color: #666666 !important;
            font-size: 13px;
        }
        .drop-zone {
            border: 1px dashed #999999;
            border-radius: 8px;
            padding: 22px 12px;
            text-align: center;
            color: #555555;
            background: #fafafa;
            cursor: pointer;
        }
        .drop-zone.drag-over {
            background: #eeeeee;
            border-color: #000000;
        }
        .drop-zone p {
            margin: 8px 0 0;
            font-size: 14px;
            color: #555555 !important;
        }
        .drop-zone input {
            display: none;
        }
        .drop-zone .browse {
            margin-top: 12px;
            background: #000000 !important;
            color: #ffffff !important;
            border: none;
            padding: 7px 18px;
            border-radius: 6px;
            cursor: pointer;
            font-size: 14px;
        }
        .upload-list {
            display: none;
            margin: 14px 0 0;
            padding: 0;
            text-align: left;
        }
        .upload-list li {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 8px 0;
            border-bottom: 1px solid #eeeeee;
            font-size: 14px;
            color: #000000;
        }
        .upload-list li img {
            width: 24px;
            height: 24px;
        }
        .upload-list li .name {
            flex: 1;
            overflow: hidden;
            white-space: nowrap;
            text-overflow: ellipsis;
            color: #000000 !important;
        }
        .upload-list li .pct {
            color: #666666 !important;
            font-size: 13px;
            white-space: nowrap;
        }
        .upload-list li .progress {
            height: 4px;
            background: #e0e0e0;
            border-radius: 4px;
            overflow: hidden;
            margin-top: 4px;
        }
        .upload-list li .progress span {
            display: block;
            height: 100%;
            width: 0%;
            background: #000000;
            transition: width 0.3s;
        }
        .upload-list li svg.cross {
            cursor: pointer;
            fill: #666666;
            flex-shrink: 0;
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
    <div class="navbar">
        <button onclick="goHome()"><i class="fa fa-home"></i> Home</button>
        <button onclick="goBack()"><i class="fa fa-backward"></i> Back</button>
        <button onclick="upload()"><i class="fa-solid fa-cloud-arrow-up"></i> Upload</button>
    </div>
    <div class="user-actions">
        <button onclick="goProfile()"><i class="fa-solid fa-user"></i> {{ user }}</button>
        <button onclick="logOut()"><i class="fa-solid fa-sign-out"></i> Logout</button>
    </div>
    <div id="uploadModal" class="upload-overlay">
        <div class="upload-modal">
            <button class="close" onclick="closeUploadModal()" title="Close"><i class="fa-solid fa-xmark"></i></button>
            <h3>Upload Files</h3>
            <p class="sub">PDF, Images, Videos and Subtitles are allowed</p>
            <div class="drop-zone" id="dropZone">
                <i class="fa-solid fa-cloud-arrow-up" style="font-size: 28px; color: #666666;"></i>
                <p>Drag &amp; drop files here</p>
                <button class="browse" type="button">Browse Files</button>
                <input type="file" id="uploadInput" multiple>
            </div>
            <ul class="upload-list" id="uploadList"></ul>
        </div>
    </div>
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
        const BASE_URL = "{{ base_url }}";
        function prefixed(path) { return BASE_URL === "/" ? path : BASE_URL + path; }
        function goHome() {
            window.location.href = prefixed("/home");
        }
        function goProfile() {
            window.location.href = prefixed('/profile');
        }
        function logOut() {
            window.location.href = prefixed("/logout");
        }
        function upload() {
            openUploadModal();
        }
        function goBack() {
            window.history.back();
        }
    </script>
    <script>
        let uploadInput = document.getElementById('uploadInput');
        let dropZone = document.getElementById('dropZone');
        let uploadList = document.getElementById('uploadList');

        let uploadsDone = 0;

        function openUploadModal() {
            document.getElementById('uploadModal').style.display = 'flex';
        }

        function closeUploadModal() {
            document.getElementById('uploadModal').style.display = 'none';
            if (uploadsDone > 0) {
                window.location.reload();
            }
        }

        // Check the file type
        function typeValidation(type) {
            let splitType = type.split('/')[0]
            if (type === 'application/pdf' || type === 'text/vtt' || splitType === 'image' || splitType === 'video') {
                return true
            }
        }

        function handleUploadFiles(files) {
            [...files].forEach((file) => {
                if (typeValidation(file.type)) {
                    uploadFile(file)
                }
            })
        }

        // upload file function
        function uploadFile(file) {
            let li = document.createElement('li')
            li.innerHTML = `
                <img src="https://thevickypedia.github.io/open-source/images/icons/${iconSelector(file.type)}" alt="">
                <div style="flex: 1; min-width: 0;">
                    <div class="name">${file.name}</div>
                    <div class="progress"><span></span></div>
                </div>
                <span class="pct">0%</span>
                <svg xmlns="http://www.w3.org/2000/svg" class="cross" height="18" width="18"><path d="m5.979 14.917-.854-.896 4-4.021-4-4.062.854-.896 4.042 4.062 4-4.062.854.896-4 4.062 4 4.021-.854.896-4-4.063Z"/></svg>
            `
            uploadList.style.display = 'block'
            uploadList.prepend(li)
            let http = new XMLHttpRequest()
            let data = new FormData()
            data.append('file', file)
            http.onload = () => {
                if (http.status === 200) {
                    // Successful response from the server
                    li.querySelector('.pct').innerHTML = 'Done'
                    li.querySelector('.progress span').style.width = '100%'
                    li.querySelector('.cross').remove()
                    uploadsDone += 1
                } else {
                    // Handle error responses
                    alert('Error uploading file. Status:' + http.status);
                    return false;
                }
            }
            http.onerror = (error) => {
                // Handle network errors
                console.log(error);
                alert('Network error during file upload.');
                return false;
            };
            http.upload.onprogress = (e) => {
                let percent_complete = (e.loaded / e.total) * 100
                li.querySelector('.pct').innerHTML = Math.round(percent_complete) + '%'
                li.querySelector('.progress span').style.width = percent_complete + '%'
            }
            http.open('POST', window.location.origin + prefixed('/upload'), true);  // asynchronous session
            http.send(data)
            li.querySelector('.cross').onclick = () => http.abort()
            http.onabort = () => {
                li.querySelector('.pct').innerHTML = 'ABORTED';
                li.querySelector('.cross').remove();
            }
        }

        // find icon for file
        function iconSelector(type) {
            let splitType = (type.split('/')[0] === 'application') ? type.split('/')[1] : type.split('/')[0];
            return splitType + '.png'
        }

        uploadInput.onchange = () => {
            handleUploadFiles(uploadInput.files)
            uploadInput.value = ''
        }
        dropZone.onclick = () => uploadInput.click()
        dropZone.ondragover = (e) => {
            e.preventDefault();
            dropZone.classList.add('drag-over');
        }
        dropZone.ondragleave = () => dropZone.classList.remove('drag-over')
        dropZone.ondrop = (e) => {
            e.preventDefault();
            dropZone.classList.remove('drag-over')
            if (e.dataTransfer.files) {
                handleUploadFiles(e.dataTransfer.files)
            }
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
