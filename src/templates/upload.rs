/// Get the HTML content to render the upload popup.
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
            background-color: #151515;
            margin: 0;
            padding: 0;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
        }
        .modal {
            width: 100%;
            max-width: 460px;
            background-color: white;
            border-radius: 16px;
            box-shadow: rgba(255, 255, 255, 0.1) 0 1px 1px 0 inset, rgba(50, 50, 93, 0.25) 0 50px 100px -20px, rgba(0, 0, 0, 0.3) 0 30px 60px -30px;
            margin: 20px;
            position: relative;
        }
        .modal .close {
            position: absolute;
            top: 12px;
            right: 16px;
            background: transparent;
            border: none;
            font-size: 18px;
            cursor: pointer;
            color: #666666;
        }
        .header-section {
            padding: 25px 0 5px;
        }
        .header-section h1 {
            font-weight: 500;
            font-size: 1.5rem;
            text-transform: uppercase;
            color: #666666;
            margin: 0;
            margin-bottom: 8px;
            text-align: center;
        }
        .header-section p {
            margin: 5px;
            font-size: 0.95rem;
            color: #666666;
            text-align: center;
        }
        .drop-section {
            min-height: 180px;
            border: 1px dashed #999999;
            background-color: #ffffff;
            margin: 15px 30px 30px 30px;
            border-radius: 12px;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            text-align: center;
            cursor: pointer;
        }
        .drop-section .cloud-icon {
            margin-top: 15px;
            margin-bottom: 10px;
        }
        .drop-section span {
            display: block;
            margin: auto;
            color: #666666;
            margin-bottom: 10px;
        }
        .drop-section button {
            color: white;
            background-color: #000000;
            border: none;
            outline: none;
            padding: 7px 20px;
            border-radius: 8px;
            margin-top: 5px;
            margin-bottom: 15px;
            cursor: pointer;
            box-shadow: rgba(50, 50, 93, 0.25) 0 13px 27px -5px, rgba(0, 0, 0, 0.3) 0 8px 16px -8px;
        }
        .drop-section input {
            display: none;
        }
        .drag-over-effect {
            background-color: #eeeeee;
            border-color: #000000;
        }
        .list-section {
            display: none;
            text-align: left;
            margin: 0 30px;
            padding-bottom: 20px;
        }
        .list-section .list-title {
            font-size: 0.95rem;
            color: #666666;
        }
        .list-section li {
            display: flex;
            margin: 12px 0;
            padding: 6px 0;
        }
        .list-section li .col:nth-child(1) {
            flex: .15;
            text-align: center;
        }
        .list-section li .col:nth-child(2) {
            flex: .75;
            text-align: left;
            font-size: 0.9rem;
            color: #000000;
            padding: 4px 10px;
        }
        .list-section li .col:nth-child(2) div.name {
            overflow: hidden;
            white-space: nowrap;
            text-overflow: ellipsis;
            max-width: 250px;
            display: inline-block;
        }
        .list-section li .col .file-name span {
            color: #666666;
            float: right;
        }
        .list-section li .file-progress {
            width: 100%;
            height: 5px;
            margin-top: 8px;
            border-radius: 8px;
            background-color: #e0e0e0;
        }
        .list-section li .file-progress span {
            display: block;
            width: 0%;
            height: 100%;
            border-radius: 8px;
            background-color: #000000;
            transition-duration: 0.4s;
        }
        .list-section li .col .file-size {
            font-size: 0.75rem;
            margin-top: 3px;
            color: #666666;
        }
        .list-section li .col svg.cross {
            fill: #666666;
            background-color: #e0e0e0;
            position: relative;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
            border-radius: 50%;
            cursor: pointer;
        }
        .list-section li.in-prog .file-size {
            display: none;
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
<body>
    <div class="modal">
        <button class="close" onclick="closeModal()" title="Close"><i class="fa-solid fa-xmark"></i></button>
        <div class="header-section">
            <h1>Upload Files</h1>
            <p>PDF, Images, Videos and Subtitles are allowed</p>
        </div>
        <div class="drop-section">
            <div class="cloud-icon">
                <img src="https://thevickypedia.github.io/open-source/images/icons/cloud.png" alt="cloud">
            </div>
            <span>Drag &amp; Drop your files here</span>
            <span>OR</span>
            <button class="file-selector">Browse Files</button>
            <input type="file" class="file-selector-input" multiple>
        </div>
        <div class="list-section">
            <div class="list-title">Uploaded Files</div>
            <div class="list"></div>
        </div>
    </div>
    <script>
        const BASE_URL = "{{ base_url }}";
        function prefixed(path) { return BASE_URL === "/" ? path : BASE_URL + path; }
        function closeModal() {
            if (window.history.length > 1) {
                window.history.back();
            } else {
                window.location.href = prefixed("/home");
            }
        }

        const dropArea = document.querySelector('.drop-section')
        const listSection = document.querySelector('.list-section')
        const listContainer = document.querySelector('.list')
        const fileSelectorInput = document.querySelector('.file-selector-input')

        // Click anywhere on the drop area (including the browse button) to pick files
        dropArea.onclick = () => fileSelectorInput.click()
        fileSelectorInput.onchange = () => {
            [...fileSelectorInput.files].forEach((file) => {
                if (typeValidation(file.type)) {
                    uploadFile(file)
                }
            })
            fileSelectorInput.value = ''
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
                </div>
            `
            listContainer.prepend(li)
            let http = new XMLHttpRequest()
            let data = new FormData()
            data.append('file', file)
            http.onload = () => {
                if (http.status === 200) {
                    // Successful response from the server
                    li.querySelectorAll('span')[0].innerHTML = 'DONE';
                    li.querySelectorAll('span')[1].style.width = '100%';
                    li.querySelector('.cross').remove();
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
                li.querySelectorAll('span')[0].innerHTML = Math.round(percent_complete) + '%'
                li.querySelectorAll('span')[1].style.width = percent_complete + '%'
            }
            http.open('POST', window.location.origin + prefixed('/upload'), true);  // asynchronous session
            http.send(data)
            li.querySelector('.cross').onclick = () => http.abort()
            http.onabort = () => {
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
</body>
</html>"#.to_string()
}
