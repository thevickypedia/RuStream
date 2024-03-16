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
        small {
            font-size: 16px;
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
        a:hover, a:active { font-size: 102%; opacity: 0.5; }
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
    <br><br><br><br>
    <!-- Context menu template (hidden by default) -->
    <div id="contextMenu" class="context-menu icon" style="display: none;">
        <div class="context-menu-item" onclick="editItem(currentPath, 'delete')"><i class="fa-regular fa-trash-can"></i>&nbsp;&nbsp;Delete</div>
        <div class="context-menu-item" onclick="editItem(currentPath, 'rename')"><i class="fa-solid fa-pen"></i></i>&nbsp;&nbsp;Rename</div>
    </div>
    {% if custom_title %}
        <h1>{{ custom_title }}</h1>
    {% else %}
        <h1>Welcome to RuStream <small>v{{ version }}</small></h1>
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
                {% if secure_path == 'true' %}
                    <li><i class="{{ file.font }}"></i>&nbsp;&nbsp;<a oncontextmenu="showContextMenu(event, '{{ file.path }}')" href="{{ file.path }}">{{ file.name }}</a></li>
                {% else %}
                    <li><i class="{{ file.font }}"></i>&nbsp;&nbsp;<a href="{{ file.path }}">{{ file.name }}</a></li>
                {% endif %}
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
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a oncontextmenu="showContextMenu(event, '{{ directory.path }}', true)" href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
    {% else %}
        <h3 style="text-align: center">No content was rendered by the server</h3>
    {% endif %}
    <hr>
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
            http.open('POST', window.location.origin + `/edit`, true);  // asynchronous session
            http.setRequestHeader('Content-Type', 'application/json'); // Set content type to JSON
            http.setRequestHeader('edit-action', action);
            http.onreadystatechange = function() {
                if (http.readyState === XMLHttpRequest.DONE) {
                    if (http.status === 200) {
                        window.location.reload();
                    } else {
                        if (http.responseText !== "") {
                            alert(`Error: ${http.responseText}`);
                        } else {
                            alert(`Error: ${http.statusText}`);
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
                alert(`New name is the same as old\n\n'${oldName}'=='${newName}'`);
            }
            // Condition 2 - Validate if the new filename starts or ends with . or _
            if (newName.startsWith('_') || newName.endsWith('_') ||
                newName.startsWith('.') || newName.endsWith('.')) {
                alert(`New name cannot start or end with '.' or '_'\n\n${newName}`);
                return false;
            }
            // Condition 3 - Validate if the new filename and the old has the same file extension.
            const oldExtension = oldName.split('.').pop();
            const newExtension = newName.split('.').pop();
            // Check condition 3
            if (oldExtension !== newExtension) {
                alert(`File extension cannot be changed\n\n'${newExtension}' => '${oldExtension}'`);
                return false;
            }
            // Condition 4 - Validate if the new filename has at least one character, apart from the file extension.
            if (newName.length <= oldExtension.length + 1) {
                alert(`At least one character is required as filename\n\nReceived ${newName.length}`);
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
