/// Get the HTML content to render the profile page.
///
/// # See Also
///
/// - This page is served as a response for the `/profile` entry point.
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
    <meta http-equiv="Cache-Control" content="no-cache, no-store, must-revalidate">
    <meta http-equiv="Pragma" content="no-cache">
    <meta http-equiv="Expires" content="0">
    <title>RuStream - Self-hosted Streaming Engine - v{{ version }}</title>
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
        .navbar {
            position: relative;
            display: flex;
            align-items: center;
            gap: 10px;
            margin: 0.5% 0 1.5%;
        }
        .navbar button {
            border: none;
            padding: 8px 14px;
            font-size: 15px;
            cursor: pointer;
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
            background: transparent;
            border: none;
            font-size: 18px;
            cursor: pointer;
            color: #666666;
        }
        .upload-modal h3 {
            margin: 0 0 4px;
            text-align: center;
            color: #000000;
        }
        .upload-modal p.sub {
            margin: 0 0 14px;
            text-align: center;
            color: #666666;
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
            color: #555555;
        }
        .drop-zone input {
            display: none;
        }
        .drop-zone .browse {
            margin-top: 12px;
            background: #000000;
            color: #ffffff;
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
            color: #000000;
        }
        .upload-list li .pct {
            color: #666666;
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
        .night .upload-modal {
            background: #1e1e1e;
        }
        .night .upload-modal h3,
        .night .upload-modal .name {
            color: #f0f0f0 !important;
        }
        .night .upload-modal p.sub,
        .night .upload-modal .pct {
            color: #aaaaaa !important;
        }
        .night .upload-modal .close {
            color: #cccccc;
        }
        .night .drop-zone {
            background: #2a2a2a;
            border-color: #666666;
        }
        .night .drop-zone p,
        .night .drop-zone {
            color: #cccccc !important;
        }
        .night .drop-zone.drag-over {
            background: #333333;
            border-color: #f0f0f0;
        }
        .night .drop-zone .browse {
            background: #f0f0f0;
            color: #000000 !important;
        }
        .night .upload-list li {
            border-color: #333333;
            color: #f0f0f0 !important;
        }
        .night .upload-list li .progress {
            background: #444444;
        }
        .night .upload-list li .progress span {
            background: #f0f0f0;
        }
    </style>
    <!-- Title list CSS -->
    <style>
        a:hover, a:active { font-size: 120%; opacity: 0.7; }
        a:link { color: blue; }
        a:visited { color: blue; }
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
<body translate="no" onload="displayTimer(); displayExpiryUTC(); displayExpiryLocal()">
<div class="toggler fa fa-moon-o"></div>
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
<hr>
<br><br>
<h4 style="text-align: center">Welcome {{ user }}</h3>
<h4>Session Validity</h4>
<p id="secondsCountDown"><p>
<p id="validityUTC"></h5>
<p id="validityLocal"></h5>
{% if file %}
    <h4>Last Accessed</h4>
    <i class="{{ file.font }}"></i>&nbsp;&nbsp;<a href="{{ file.path }}">{{ file.name }}</a>
{% endif %}
<script>
    const BASE_URL = "{{ base_url }}";
    function prefixed(path) { return BASE_URL === "/" ? path : BASE_URL + path; }
    function goHome() { window.location.href = prefixed("/home"); }
    function goProfile() { window.location.href = prefixed('/profile'); }
    function logOut() { window.location.href = prefixed("/logout"); }
    function upload() { openUploadModal(); }
    function goBack() { window.history.back(); }
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
    function secondsToStr(seconds) {
        let levels = [
            [Math.floor(seconds / 31536000), 'years'],
            [Math.floor((seconds % 31536000) / 86400), 'days'],
            [Math.floor(((seconds % 31536000) % 86400) / 3600), 'hours'],
            [Math.floor((((seconds % 31536000) % 86400) % 3600) / 60), 'minutes'],
            [Math.floor((((seconds % 31536000) % 86400) % 3600) % 60), 'seconds'],
        ];
        let returntext = '';

        for (let i = 0, max = levels.length; i < max; i++) {
            if (levels[i][0] === 0) continue;
            returntext += ', ' + levels[i][0] + ' ' + (levels[i][0] === 1 ? levels[i][1].substr(0, levels[i][1].length - 1) : levels[i][1]);
        }
        return returntext.trim();
    }

    function displayTimer() {
        let seconds = '{{ time_left }}';
        let display = document.getElementById('secondsCountDown');
        display.innerHTML = secondsToStr(seconds).substring(1);
        let countdown = setInterval(function () {
            seconds--;
            display.innerHTML = secondsToStr(seconds).substring(1) + "s";
            if (seconds <= 0) {
                clearInterval(countdown);
            }
        }, 1000);
    }
</script>
<script>
    function displayExpiryUTC() {
        let seconds = '{{ time_left }}';
        let expiryTime = new Date(Date.now() + seconds * 1000);
        let display = document.getElementById('validityUTC');
        display.innerHTML = expiryTime.toUTCString();
    }
</script>
<script>
    function displayExpiryLocal() {
        let seconds = '{{ time_left }}';
        let expiryTime = new Date(Date.now() + seconds * 1000);
        let display = document.getElementById('validityLocal');
        // Options for date and time formatting
        let options = {
            weekday: 'short',
            day: '2-digit',
            month: 'short',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            timeZoneName: 'short'
        };
        display.innerHTML = expiryTime.toLocaleString('en-US', options);
    }

    setInterval(function () {
        window.location.href = prefixed("/");
    }, {{ time_left }} * 1000 - 1000); // Convert time_left to milliseconds and subract 1 second
</script>
</body>
</html>
"###.to_string()
}
