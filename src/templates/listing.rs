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
    <!-- Navbar CSS -->
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
        small {
            font-size: 16px;
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
    <!-- Title and listing CSS -->
    <style>
        a:hover, a:active { font-size: 102%; opacity: 0.5; }
        a:link { color: blue; }
        a:visited { color: blue; }
        ol {
            list-style: none;
            padding-left: 0;
        }
        li {
            margin: 0.6rem 0;
            list-style-type: none; /* Hide default marker */
        }
        .file-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            padding: 0.4rem 0.6rem;
        }
        .file-row:hover {
            background: rgba(65, 105, 225, 0.08);
        }
        .file-info {
            display: flex;
            align-items: center;
            gap: 6px;
            flex: 1;
            min-width: 0;
        }
        .file-info a {
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .file-meta {
            color: #888888;
            font-size: 0.85em;
            white-space: nowrap;
        }
        .file-actions {
            display: flex;
            align-items: center;
            gap: 6px;
            flex-shrink: 0;
        }
        .file-actions button {
            border: 1px solid #cccccc;
            background: transparent;
            padding: 4px 10px;
            font-size: 13px;
            cursor: pointer;
            border-radius: 4px;
        }
    </style>
    <!-- Convert dialog and toast CSS -->
    <style>
        .modal-overlay {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.4);
            align-items: center;
            justify-content: center;
            z-index: 999;
        }
        .modal {
            background: #ffffff;
            border-radius: 8px;
            padding: 20px 24px;
            min-width: 320px;
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.25);
        }
        .modal h3 {
            margin: 0 0 8px;
        }
        .modal .file-name {
            color: #555555;
            margin: 0 0 12px;
            word-break: break-all;
        }
        .modal select {
            width: 100%;
            padding: 8px;
            margin: 8px 0 16px;
            font-size: 15px;
            border: 1px solid #cccccc;
            border-radius: 4px;
            box-sizing: border-box;
        }
        .modal input {
            width: 100%;
            padding: 8px;
            margin: 8px 0 16px;
            font-size: 15px;
            border: 1px solid #cccccc;
            border-radius: 4px;
            box-sizing: border-box;
        }
        .modal-actions {
            display: flex;
            gap: 8px;
            justify-content: flex-end;
        }
        .modal-actions button {
            border: 1px solid #cccccc;
            background: transparent;
            padding: 6px 14px;
            font-size: 14px;
            cursor: pointer;
            border-radius: 4px;
        }
        .info-table {
            width: 100%;
            border-collapse: collapse;
            font-size: 14px;
            margin: 4px 0 12px;
        }
        .info-table td {
            padding: 6px 8px;
            border-bottom: 1px solid #eeeeee;
            vertical-align: top;
            word-break: break-all;
        }
        .info-table td:first-child {
            color: #666666;
            white-space: nowrap;
            width: 130px;
        }
        .toast {
            display: none;
            position: fixed;
            bottom: 30px;
            left: 50%;
            transform: translateX(-50%);
            padding: 12px 20px;
            border-radius: 6px;
            color: #ffffff;
            font-size: 15px;
            max-width: 90%;
            word-break: break-word;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            z-index: 1000;
        }
        .toast.success { background: #2e7d32; }
        .toast.error { background: #c62828; }
        .toast.info { background: #1565c0; }
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
        .night .modal {
            background: #1e1e1e;
        }
        .night .modal h3,
        .night .modal .file-name {
            color: #f0f0f0 !important;
        }
        .night .modal select,
        .night .modal input {
            background: #2a2a2a;
            color: #f0f0f0 !important;
            border-color: #555555;
        }
        .night .modal-actions button {
            color: #f0f0f0 !important;
            border-color: #555555;
        }
        .night .info-table td {
            border-color: #333333;
        }
        .night .info-table td:first-child {
            color: #aaaaaa !important;
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
    <style>
        /* Style for context menu */
        .context-menu {
            position: absolute;
            background-color: #fff;
            border: 1px solid #ccc;
            padding: 5px 0;
            box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.2);
        }
        .context-menu-item {
            padding: 5px 10px;
            cursor: pointer;
            background-color: #fff !important; /* White background */
            color: #000 !important; /* Black font */
        }
        .context-menu-item:hover {
            background-color: #000 !important; /* Black background */
            color: #fff !important; /* White font */
        }
        .icon {
            background-color: #fff !important; /* White background */
            color: #000 !important; /* Black font */
        }
        .icon:hover {
            background-color: #000 !important; /* Black background */
            color: #fff !important; /* White font */
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
    <div class="navbar">
        <button onclick="goHome()"><i class="fa fa-home"></i> Home</button>
        <button onclick="goBack()"><i class="fa fa-backward"></i> Back</button>
        <button onclick="upload()"><i class="fa-solid fa-cloud-arrow-up"></i> Upload</button>
    </div>
    <div class="user-actions">
        <button onclick="goProfile()"><i class="fa-solid fa-user"></i> {{ user }}</button>
        <button onclick="logOut()"><i class="fa-solid fa-sign-out"></i> Logout</button>
    </div>
    <!-- Convert dialog (hidden by default) -->
    <div id="convertDialog" class="modal-overlay">
        <div class="modal">
            <h3>Convert video</h3>
            <p class="file-name" id="convertFileName"></p>
            <label for="convertFormat">Convert to:</label>
            <select id="convertFormat">
                {% for format in video_formats %}
                    <option value="{{ format }}">{{ format }}</option>
                {% endfor %}
            </select>
            <div class="modal-actions">
                <button onclick="startConversion()"><i class="fa-solid fa-wand-magic-sparkles"></i>&nbsp;&nbsp;Convert</button>
                <button onclick="closeConvertDialog()">Cancel</button>
            </div>
        </div>
    </div>
    <!-- Rename dialog (hidden by default) -->
    <div id="renameDialog" class="modal-overlay">
        <div class="modal">
            <h3>Rename file</h3>
            <label for="renameInput">New file name:</label>
            <input type="text" id="renameInput" spellcheck="false"
                   onkeydown="if (event.key === 'Enter') submitRename(); if (event.key === 'Escape') closeRenameDialog();">
            <div class="modal-actions">
                <button onclick="submitRename()"><i class="fa-solid fa-pen"></i>&nbsp;&nbsp;Save</button>
                <button onclick="closeRenameDialog()">Cancel</button>
            </div>
        </div>
    </div>
    <!-- Info dialog (hidden by default) -->
    <div id="infoDialog" class="modal-overlay">
        <div class="modal">
            <h3>File Info</h3>
            <table class="info-table" id="infoContent"></table>
            <div class="modal-actions">
                <button onclick="closeInfoDialog()">Close</button>
            </div>
        </div>
    </div>
    <!-- Upload dialog (hidden by default) -->
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
    <!-- Toast notification (hidden by default) -->
    <div id="toast" class="toast"></div>
    <!-- Context menu template (hidden by default) -->
    <div id="contextMenu" class="context-menu icon" style="display: none;">
        <div class="context-menu-item" onclick="editItem(currentPath, 'delete')"><i class="fa-regular fa-trash-can"></i>&nbsp;&nbsp;Delete</div>
        <div class="context-menu-item" onclick="openRenameDialog(currentPath)"><i class="fa-solid fa-pen"></i>&nbsp;&nbsp;Rename</div>
    </div>
    {% if custom_title %}
        <h1>{{ custom_title }}</h1>
    {% else %}
        <h1>Welcome to RuStream <small>v{{ version }}</small></h1>
    {% endif %}
    <hr>
    {% if dir_name or files or directories %}
        <!-- Display directory name if within subdir -->
        {% if dir_name %}
            <h3>{{ dir_name }}</h3>
        {% endif %}
        <!-- Display number of files and list the files -->
        {% if files %}
            <h3>Files {{ files|length }}</h3>
            <ol>
                {% for file in files %}
                    <li class="file-row">
                        <div class="file-info">
                            <i class="{{ file.font }}"></i>&nbsp;&nbsp;<a href="{{ file.path }}">{{ file.name }}</a>
                            {% if file.size or file.duration %}
                                <span class="file-meta">
                                    {% if file.size %}{{ file.size }}{% endif %}
                                    {% if file.duration %} &middot; {{ file.duration }}{% endif %}
                                </span>
                            {% endif %}
                        </div>
                        <div class="file-actions">
                            {% if ffmpeg_enabled and file.video == 'true' %}
                                <button onclick="openConvertDialog('{{ file.path }}')" title="Convert format"><i class="fa-solid fa-wand-magic-sparkles"></i>&nbsp;&nbsp;Convert</button>
                            {% endif %}
                            <button onclick="downloadFile('{{ file.path }}')" title="Download"><i class="fa-solid fa-download"></i></button>
                            <button onclick="openRenameDialog('{{ file.path }}')" title="Rename"><i class="fa-solid fa-pen"></i></button>
                            <button onclick="editItem('{{ file.path }}', 'delete')" title="Delete"><i class="fa-regular fa-trash-can"></i></button>
                            <button onclick="showFileInfo('{{ file.path }}')" title="Info"><i class="fa-solid fa-circle-info"></i></button>
                        </div>
                    </li>
                {% endfor %}
            </ol>
        {% endif %}
        <!-- Display number of directories and list the directories -->
        {% if directories %}
            <h3>Directories {{ directories|length }}</h3>
            {% for directory in directories %}
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
    {% else %}
        <h3 style="text-align: center">No content was rendered by the server</h3>
    {% endif %}
    <hr>
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
        function showFileInfo(path) {
            let filePath = path.replace(/^stream\//, '');
            let http = new XMLHttpRequest();
            http.open('GET', window.location.origin + prefixed('/info?file=') + encodeURIComponent(filePath), true);
            http.onreadystatechange = function() {
                if (http.readyState !== XMLHttpRequest.DONE) {
                    return;
                }
                if (http.status === 200) {
                    let info = JSON.parse(http.responseText);
                    let rows = [
                        ['Name', info.name],
                        ['Path', info.path],
                        ['Type', info.kind],
                        ['Size', info.size],
                        ['Format', info.format],
                        ['Duration', info.duration],
                        ['Created', info.created],
                        ['Modified', info.modified],
                        ['Permissions', info.permissions],
                        ['Owner (UID:GID)', info.owner]
                    ];
                    let html = '';
                    rows.forEach(function(row) {
                        html += '<tr><td>' + row[0] + '</td><td>' + row[1] + '</td></tr>';
                    });
                    document.getElementById('infoContent').innerHTML = html;
                    document.getElementById('infoDialog').style.display = 'flex';
                } else {
                    if (http.responseText !== "") {
                        showToast(`Error: ${http.responseText}`, 'error');
                    } else {
                        showToast(`Error: ${http.statusText}`, 'error');
                    }
                }
            };
            http.send();
        }

        function closeInfoDialog() {
            document.getElementById('infoDialog').style.display = 'none';
        }
    </script>
    <script>
        function downloadFile(path) {
            let filePath = path.replace(/^stream\//, '');
            window.location.href = window.location.origin + prefixed('/download?file=') + encodeURIComponent(filePath);
        }

        const VIDEO_FORMATS = [{% for format in video_formats %}'{{ format }}',{% endfor %}];

        let convertTargetPath = null;
        let convertTimer = null;

        function showToast(message, type) {
            let toast = document.getElementById('toast');
            toast.textContent = message;
            toast.className = 'toast ' + (type || 'info');
            toast.style.display = 'block';
            clearTimeout(convertTimer);
            convertTimer = setTimeout(function() {
                toast.style.display = 'none';
            }, 5000);
        }

        function openConvertDialog(path) {
            let fileName = extractFileName(path);
            let currentFormat = fileName.split('.').pop().toLowerCase();
            let select = document.getElementById('convertFormat');
            select.innerHTML = '';
            VIDEO_FORMATS.forEach(function(format) {
                if (format === currentFormat) {
                    return;
                }
                let option = document.createElement('option');
                option.value = format;
                option.text = format;
                select.appendChild(option);
            });
            if (select.options.length === 0) {
                showToast(`No other format is available for '${currentFormat}'`, 'error');
                return;
            }
            convertTargetPath = path;
            document.getElementById('convertFileName').innerText = fileName;
            document.getElementById('convertDialog').style.display = 'flex';
        }

        function closeConvertDialog() {
            document.getElementById('convertDialog').style.display = 'none';
        }

        function startConversion() {
            let format = document.getElementById('convertFormat').value;
            if (!format || convertTargetPath === null) {
                return;
            }
            closeConvertDialog();
            let fileName = extractFileName(convertTargetPath);
            let trueURL = window.location.href + '/' + fileName;
            let http = new XMLHttpRequest();
            http.open('POST', window.location.origin + prefixed('/convert'), true);
            http.setRequestHeader('Content-Type', 'application/json');
            http.onreadystatechange = function() {
                if (http.readyState === XMLHttpRequest.DONE) {
                    if (http.status === 202) {
                        let response = JSON.parse(http.responseText);
                        showToast(`Converting '${fileName}' to ${format}...`, 'info');
                        pollConversion(response.job, fileName, format);
                    } else {
                        if (http.responseText !== "") {
                            showToast(`Error: ${http.responseText}`, 'error');
                        } else {
                            showToast(`Error: ${http.statusText}`, 'error');
                        }
                    }
                }
            };
            http.send(JSON.stringify({
                url_locator: trueURL,
                path_locator: convertTargetPath,
                new_format: format
            }));
        }

        let renameTargetPath = null;

        function pollConversion(jobId, fileName, format) {
            let poll = setInterval(function() {
                let status = new XMLHttpRequest();
                status.open('GET', window.location.origin + prefixed('/convert/status/') + jobId, true);
                status.onreadystatechange = function() {
                    if (status.readyState !== XMLHttpRequest.DONE) {
                        return;
                    }
                    if (status.status === 200) {
                        let job = JSON.parse(status.responseText);
                        if (job.state === 'running') {
                            return;
                        }
                        clearInterval(poll);
                        if (job.state === 'done') {
                            showToast(`'${fileName}' was converted to ${format} successfully`, 'success');
                            setTimeout(function() {
                                window.location.reload();
                            }, 2000);
                        } else {
                            showToast(`Conversion failed: ${job.detail}`, 'error');
                        }
                    } else {
                        clearInterval(poll);
                        showToast('Failed to fetch conversion status', 'error');
                    }
                };
                status.send();
            }, 1500);
        }

        function openRenameDialog(path) {
            renameTargetPath = path;
            let input = document.getElementById('renameInput');
            input.value = extractFileName(path);
            document.getElementById('renameDialog').style.display = 'flex';
            input.focus();
            input.select();
        }

        function closeRenameDialog() {
            document.getElementById('renameDialog').style.display = 'none';
        }

        function submitRename() {
            if (renameTargetPath === null) {
                return;
            }
            let newName = document.getElementById('renameInput').value;
            let fileName = extractFileName(renameTargetPath);
            if (!isValidName(fileName, newName)) {
                return;
            }
            closeRenameDialog();
            let trueURL = window.location.href + '/' + fileName;
            editAction('rename', trueURL, renameTargetPath, newName);
        }
    </script>
    <script>
        var contextMenu = document.getElementById('contextMenu');

        // Function to show context menu
        function showContextMenu(event, path, isDir = false) {
            event.preventDefault();

            // Set the global variable to the current file path
            currentPath = path;
            directory = isDir;

            // Calculate the appropriate coordinates for the context menu
            var mouseX = event.clientX;
            var mouseY = event.clientY;
            var windowWidth = window.innerWidth;
            var windowHeight = window.innerHeight;
            var contextMenuWidth = contextMenu.offsetWidth;
            var contextMenuHeight = contextMenu.offsetHeight;
            var scrollX = window.scrollX || window.pageXOffset;
            var scrollY = window.scrollY || window.pageYOffset;

            // Adjust the coordinates considering the scroll position and moving 2 pixels away from the mouse pointer
            var menuX = mouseX + scrollX + contextMenuWidth > windowWidth ? mouseX + scrollX - contextMenuWidth - 2 : mouseX + scrollX + 2;
            var menuY = mouseY + scrollY + contextMenuHeight > windowHeight ? mouseY + scrollY - contextMenuHeight - 2 : mouseY + scrollY + 2;

            // Position the context menu at the calculated coordinates
            contextMenu.style.left = menuX + 'px';
            contextMenu.style.top = menuY + 'px';
            contextMenu.style.display = 'block';
        }

        function editAction(action, trueURL, relativePath, newName) {
            let http = new XMLHttpRequest();
            http.open('POST', window.location.origin + prefixed('/edit'), true);  // asynchronous session
            http.setRequestHeader('Content-Type', 'application/json'); // Set content type to JSON
            http.setRequestHeader('edit-action', action);
            http.onreadystatechange = function() {
                if (http.readyState === XMLHttpRequest.DONE) {
                    if (http.status === 200) {
                        window.location.reload();
                    } else {
                        if (http.responseText !== "") {
                            showToast(`Error: ${http.responseText}`, 'error');
                        } else {
                            showToast(`Error: ${http.statusText}`, 'error');
                        }
                    }
                }
            };
            let data = {
                url_locator: trueURL,
                path_locator: relativePath,
                new_name: newName
            };
            http.send(JSON.stringify(data));
        }

        function getConfirmation(fileName, action) {
            let confirmation = confirm(`Are you sure you want to ${action}?\n\n'${fileName}'`);
            if (!confirmation) {
                contextMenu.style.display = 'none';
                return false;
            }
            return true;
        }

        function extractFileName(path) {
            // Find the last occurrence of either '/' or '\'
            const lastIndex = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));

            // Extract the filename using substring
            return path.substring(lastIndex + 1);
        }

        function isValidName(oldName, newName) {
            // Condition 1 - Validate if the new filename is the same as old.
            if (oldName === newName) {
                showToast(`New name is the same as old: '${oldName}'`, 'error');
                return false;
            }
            // Condition 2 - Validate if the new filename starts or ends with . or _
            if (newName.startsWith('_') || newName.endsWith('_') ||
                newName.startsWith('.') || newName.endsWith('.')) {
                showToast(`New name cannot start or end with '.' or '_'`, 'error');
                return false;
            }
            // Condition 3 - Validate if the new filename has at least one character, apart from the file extension.
            const newExtension = newName.split('.').pop();
            if (newName.length <= newExtension.length + 1) {
                showToast('At least one character is required as filename', 'error');
                return false;
            }
            return true;
        }

        // Function to handle delete/rename action
        function editItem(relativePath, action) {
            contextMenu.style.display = 'none';

            let fileName = extractFileName(relativePath);
            if (action === 'delete') {
                let pass = getConfirmation(fileName, action);
                if (!pass) {
                    return;
                }
                var newName = null;
            } else {
                if (directory) {
                    alert("Only a 'delete' action is permitted on directories");
                    return;
                }
                var newName = prompt(`Enter a new name for the file\n\nCurrent: ${fileName}\n`);
                if (!isValidName(fileName, newName)) {
                    return;
                }
            }
            let trueURL = window.location.href + '/' + fileName;
            editAction(action, trueURL, relativePath, newName);
        }

        // Hide context menu when clicking outside
        document.addEventListener('click', function(event) {
            if (event.target !== contextMenu && !contextMenu.contains(event.target)) {
                contextMenu.style.display = 'none';
            }
        });
        </script>
</body>
</html>
"###.to_string()
}
