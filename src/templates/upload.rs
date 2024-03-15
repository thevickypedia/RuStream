/// Get the HTML content to render the upload page.
///
/// # See Also
///
/// - This page is served as a response for the `/upload` entry point.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r#"<!DOCTYPE html>
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
    <style>
        /* Google fonts with a backup alternative */
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@400;500;700&display=swap');
        * {
            font-family: 'Ubuntu', 'PT Serif', sans-serif;
        }
        body {
            background-color: #6c7dac;
            padding: 30px;
            margin: 0;
        }
        .container {
            text-align: center;
            width: 100%;
            max-width: 500px;
            min-height: 435px;
            margin: auto;
            background-color: white;
            border-radius: 16px;
            box-shadow: rgba(255, 255, 255, 0.1) 0 1px 1px 0 inset, rgba(50, 50, 93, 0.25) 0 50px 100px -20px, rgba(0, 0, 0, 0.3) 0 30px 60px -30px;
        }
        .header-section {
            padding: 25px 0;
        }
        .header-section h1 {
            font-weight: 500;
            font-size: 1.7rem;
            text-transform: uppercase;
            color: #707EA0;
            margin: 0;
            margin-bottom: 8px;
        }
        .header-section p,
        .header-section label {
            margin: 5px;
            font-size: 0.95rem;
            color: #707EA0;
        }
        .drop-section {
            min-height: 250px;
            border: 1px dashed #A8B3E3;
            background-image: linear-gradient(180deg, white, #F1F6FF);
            margin: 5px 35px 35px 35px;
            border-radius: 12px;
            position: relative;
        }
        .drop-section div.col:first-child {
            opacity: 1;
            visibility: visible;
            transition-duration: 0.2s;
            transform: scale(1);
            width: 200px;
            margin: auto;
        }
        .drop-section div.col:last-child {
            font-size: 40px;
            font-weight: 700;
            color: #c0cae1;
            position: absolute;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
            margin: auto;
            width: 200px;
            height: 55px;
            pointer-events: none;
            opacity: 0;
            visibility: hidden;
            transform: scale(0.6);
            transition-duration: 0.2s;
        }
        /* use "drag-over-effect" class in js */
        .drag-over-effect div.col:first-child {
            opacity: 0;
            visibility: hidden;
            pointer-events: none;
            transform: scale(1.1);
        }
        .drag-over-effect div.col:last-child {
            opacity: 1;
            visibility: visible;
            transform: scale(1);
        }
        .drop-section .cloud-icon {
            margin-top: 25px;
            margin-bottom: 20px;
        }
        .drop-section span,
        .drop-section button {
            display: block;
            margin: auto;
            color: #707EA0;
            margin-bottom: 10px;
        }
        .drop-section button {
            color: white;
            background-color: #5874C6;
            border: none;
            outline: none;
            padding: 7px 20px;
            border-radius: 8px;
            margin-top: 20px;
            cursor: pointer;
            box-shadow: rgba(50, 50, 93, 0.25) 0 13px 27px -5px, rgba(0, 0, 0, 0.3) 0 8px 16px -8px;
        }
        .drop-section input {
            display: none;
        }
        .list-section {
            display: none;
            text-align: left;
            margin: 0 35px;
            padding-bottom: 20px;
        }
        .list-section .list-title {
            font-size: 0.95rem;
            color: #707EA0;
        }
        .list-section li {
            display: flex;
            margin: 15px 0;
            padding-top: 4px;
            padding-bottom: 2px;
            border-radius: 8px;
            transition-duration: 0.2s;
        }
        .list-section li:hover {
            box-shadow: #E3EAF9 0 0 4px 0, #E3EAF9 0 12px 16px 0;
        }
        .list-section li .col {
            flex: .1;
        }
        .list-section li .col:nth-child(1) {
            flex: .15;
            text-align: center;
        }
        .list-section li .col:nth-child(2) {
            flex: .75;
            text-align: left;
            font-size: 0.9rem;
            color: #3e4046;
            padding: 8px 10px;
        }
        .list-section li .col:nth-child(2) div.name {
            overflow: hidden;
            white-space: nowrap;
            text-overflow: ellipsis;
            max-width: 250px;
            display: inline-block;
        }
        .list-section li .col .file-name span {
            color: #707EA0;
            float: right;
        }
        .list-section li .file-progress {
            width: 100%;
            height: 5px;
            margin-top: 8px;
            border-radius: 8px;
            background-color: #dee6fd;
        }
        .list-section li .file-progress span {
            display: block;
            width: 0%;
            height: 100%;
            border-radius: 8px;
            background-image: linear-gradient(120deg, #6b99fd, #9385ff);
            transition-duration: 0.4s;
        }
        .list-section li .col .file-size {
            font-size: 0.75rem;
            margin-top: 3px;
            color: #707EA0;
        }
        .list-section li .col svg.cross,
        .list-section li .col svg.tick {
            fill: #8694d2;
            background-color: #dee6fd;
            position: relative;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
            border-radius: 50%;
        }
        .list-section li .col svg.tick {
            fill: #50a156;
            background-color: transparent;
        }
        .list-section li.complete span,
        .list-section li.complete .file-progress,
        .list-section li.complete svg.cross {
            display: none;
        }
        .list-section li.in-prog .file-size,
        .list-section li.in-prog svg.tick {
            display: none;
        }
    </style>
    <style>
        a,
        button {
            color: white;
            background-color: #6c7dac;
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
<body>
    <button class="upload" onclick="upload()"><i class="fa-solid fa-cloud-arrow-up"></i> Upload</button>
    <button class="home" onclick="goHome()"><i class="fa fa-home"></i> Home</button>
    <button class="back" onclick="goBack()"><i class="fa fa-backward"></i> Back</button>
    <div class="dropdown">
        <button class="dropbtn"><i class="fa fa-user"></i></button>
        <div class="dropdown-content">
            <a onclick="goProfile()" style="cursor: pointer;"><i class="fa-solid fa-user-lock"></i> {{ user }}</a>
            <a onclick="logOut()" style="cursor: pointer"><i class="fa fa-sign-out"></i> logout</a>
        </div>
    </div>
    <br><br><br>
    <div class="container">
        <div class="header-section">
            <h1>Upload Files</h1>
            <p>PDF, Images, Videos and Subtitles are allowed</p>
            <br>
            <input type="checkbox" id="dedicated" name="dedicated" title="Files will be stored in a secured location, which can only be accessed by '{{ user }}'" checked>
            <label for="dedicated" title="Files will be stored in a secured location, which can only be accessed by '{{ user }}'"><i class="fa-solid fa-lock"></i></i>&nbsp;&nbsp;Upload files to '{{ user }}' directory</label>
        </div>
        <div class="drop-section">
            <div class="col">
                <div class="cloud-icon">
                    <img src="https://thevickypedia.github.io/open-source/images/icons/cloud.png" alt="cloud">
                </div>
                <span>Drag & Drop your files here</span>
                <span>OR</span>
                <button class="file-selector">Browse Files</button>
                <input type="file" class="file-selector-input" multiple>
            </div>
            <div class="col">
                <div class="drop-here">Drop Here</div>
            </div>
        </div>
        <div class="list-section">
            <div class="list-title">Uploaded Files</div>
            <div class="list"></div>
        </div>
    </div>
    <script>
        const dropArea = document.querySelector('.drop-section')
        const listSection = document.querySelector('.list-section')
        const listContainer = document.querySelector('.list')
        const fileSelector = document.querySelector('.file-selector')
        const fileSelectorInput = document.querySelector('.file-selector-input')

        // Upload files with browse button
        fileSelector.onclick = () => fileSelectorInput.click()
        fileSelectorInput.onchange = () => {
            [...fileSelectorInput.files].forEach((file) => {
                if (typeValidation(file.type)) {
                    uploadFile(file)
                }
            })
        }

        // Check the file type
        function typeValidation(type) {
            let splitType = type.split('/')[0]
            if (type === 'application/pdf' || type === 'text/vtt' || splitType === 'image' || splitType === 'video') {
                return true
            }
        }

        // When file is over the drag area
        dropArea.ondragover = (e) => {
            e.preventDefault();
            [...e.dataTransfer.items].forEach((item) => {
                if (typeValidation(item.type)) {
                    dropArea.classList.add('drag-over-effect')
                }
            })
        }
        // When file leave the drag area
        dropArea.ondragleave = () => {
            dropArea.classList.remove('drag-over-effect')
        }
        // When file drop on the drag area
        dropArea.ondrop = (e) => {
            e.preventDefault();
            dropArea.classList.remove('drag-over-effect')
            if (e.dataTransfer.items) {
                [...e.dataTransfer.items].forEach((item) => {
                    if (item.kind === 'file') {
                        const file = item.getAsFile();
                        if (typeValidation(file.type)) {
                            uploadFile(file)
                        }
                    }
                })
            } else {
                [...e.dataTransfer.files].forEach((file) => {
                    if (typeValidation(file.type)) {
                        uploadFile(file)
                    }
                })
            }
        }
        // upload file function
        function uploadFile(file) {
            listSection.style.display = 'block'
            let li = document.createElement('li')
            li.classList.add('in-prog')
            li.innerHTML = `
                <div class="col">
                    <img src="https://thevickypedia.github.io/open-source/images/icons/${iconSelector(file.type)}" alt="">
                </div>
                <div class="col">
                    <div class="file-name">
                        <div class="name">${file.name}</div>
                        <span>0%</span>
                    </div>
                    <div class="file-progress">
                        <span></span>
                    </div>
                    <div class="file-size">${(file.size / (1024 * 1024)).toFixed(2)} MB</div>
                </div>
                <div class="col">
                    <svg xmlns="http://www.w3.org/2000/svg" class="cross" height="20" width="20"><path d="m5.979 14.917-.854-.896 4-4.021-4-4.062.854-.896 4.042 4.062 4-4.062.854.896-4 4.062 4 4.021-.854.896-4-4.063Z"/></svg>
                    <svg xmlns="http://www.w3.org/2000/svg" class="tick" height="20" width="20"><path d="m8.229 14.438-3.896-3.917 1.438-1.438 2.458 2.459 6-6L15.667 7Z"/></svg>
                </div>
            `
            listContainer.prepend(li)
            let http = new XMLHttpRequest()
            let checkbox = document.getElementById('dedicated');
            let data = new FormData()
            data.append('file', file)
            http.onload = () => {
                checkbox.disabled = false;
                if (http.status === 200) {
                    // Successful response from the server
                    li.classList.add('complete');
                    li.classList.remove('in-prog');
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
                checkbox.disabled = false;
                return false;
            };
            http.upload.onprogress = (e) => {
                checkbox.disabled = true;
                let percent_complete = (e.loaded / e.total) * 100
                li.querySelectorAll('span')[0].innerHTML = Math.round(percent_complete) + '%'
                li.querySelectorAll('span')[1].style.width = percent_complete + '%'
            }
            http.open('POST', window.location.origin + '/upload', true);  // asynchronous session
            http.setRequestHeader('secure-flag', checkbox.checked);
            http.send(data)
            li.querySelector('.cross').onclick = () => http.abort()
            http.onabort = () => {
                checkbox.disabled = false;
                let crossElement = li.querySelector('.cross');
                // Insert a red cross sign
                crossElement.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" class="cross-solid" height="20" width="20"><path d="m5.979 14.917-.854-.896 4-4.021-4-4.062.854-.896 4.042 4.062 4-4.062.854.896-4 4.062 4 4.021-.854.896-4-4.063Z" stroke="red" stroke-width="2"></path></svg>';
                let context = li.querySelector('.file-name');
                let spanElement = context.querySelector('span');
                // Change uploaded percentage into a text
                spanElement.innerHTML = 'ABORTED!!';
            }
        }

        // find icon for file
        function iconSelector(type) {
            let splitType = (type.split('/')[0] === 'application') ? type.split('/')[1] : type.split('/')[0];
            return splitType + '.png'
        }
    </script>
    <script>
        function goHome() {
            window.location.href = "/home";
        }
        function goProfile() {
            window.location.href = '/profile';
        }
        function logOut() {
            window.location.href = "/logout";
        }
        function upload() {
            window.location.href = "/upload";
        }
        function goBack() {
            window.history.back();
        }
    </script>
    <script>
        document.getElementById("dedicated").addEventListener("change", function() {
            if (!this.checked) {
                let confirmation = confirm(
                    "Uploading to public space will not only make your uploads accessible " +
                    "to other users, but it will also overwrite any existing files " +
                    "with the same name.\n\nAre you sure you want to proceed?"
                );
                if (!confirmation) {
                    this.checked = true;
                }
            }
        });
    </script>
</body>
</html>"#.to_string()
}
