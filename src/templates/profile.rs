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
    function goHome() { window.location.href = "/home"; }
    function goProfile() { window.location.href = '/profile'; }
    function logOut() { window.location.href = "/logout"; }
    function upload() { window.location.href = "/upload"; }
    function goBack() { window.history.back(); }
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

    function setCookieWithShortExpiration(name, value, secondsToExpire) {
        // Calculate the expiration date
        var expirationDate = new Date();
        expirationDate.setTime(expirationDate.getTime() + (secondsToExpire * 1000)); // Convert seconds to milliseconds

        // Construct the cookie string
        var cookieString = name + "=" + encodeURIComponent(value);
        cookieString += "; expires=" + expirationDate.toUTCString();

        // Set the cookie
        document.cookie = cookieString;
    }

    setInterval(function () {
        setCookieWithShortExpiration("detail", "Session Expired", 5);
        window.location.href = "/error";
    }, {{ time_left }} * 1000 - 1000); // Convert time_left to milliseconds and subract 1 second
</script>
</body>
</html>
"###.to_string()
}
