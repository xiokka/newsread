pub const default_css:&str = r#"
body {
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        background-color: #2c2c2c;
        color:white;
        height: 100vh;
}
header {
        background-color: #111;
        color: white;
        padding: 10px;
        letter-spacing: 1px;
}

header h1 {
        margin: 0;
        font-size:1.5em;
}
h2, h3, p {
        font-weight: normal;
        margin-bottom: 10px;
        font-size: 1em;
}
p {
        color: #667;
}

.container {
        padding: 15px;
        gap: 15px;
}

.item {
        display: none;
        flex-direction: column;
        margin-bottom: 10px;
}
.item h2, h3, p {
        margin:0;
}
.item img {
        max-width: 100%;
        height: auto;
        border-radius: 4px;
}

a {
        color: #9ad2d8;
        text-decoration: none;
}
a:hover {
        text-decoration: underline;
}

a:visited {
        color: #ffaa29;
}

input[type="radio"] {
        display: none;
}

label {
        color: white;
        padding: 10px;
        position: relative;
        cursor: pointer;
        line-height: 20px;
        display: block;
}
.columns {
        display: flex;
        flex: 1;
        gap: 10px;
        padding: 20px;
        overflow: hidden;
        height:80%;
}
.left-column {
        flex: 1;
        height: 100%;
        overflow-y: auto;
}
.center-column {
        flex: 1.5;
        height: 100%;
        overflow-y: auto;
}
.right-column {
        flex: 2;
        height: 100%;
        background-color: white;
}
.view-container {
        position: relative;
        width: 100%;
        height: 100%;
        overflow: hidden;
}
.view-container iframe {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        border: none;
}   
"#;


pub const microblog_html:&str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<title>newsread</title>
	<style>
#all:checked ~ .container .all,
{CHANNELS_CSS}
{
	display: flex;
}

{CUSTOM_CSS}
</style>
</head>
<body>
      <header>
                <h1>newsread</h1>
      </header>
  <div class="columns">
    <div class="left-column">
      {LABELS} <!-- Dynamic channel labels go here -->
    </div>

    <!-- Center Column for Radio and Main Content -->
    <div class="center-column">
      {RADIO} <!-- Hidden radio buttons here -->
      <div class="container">
        {CONTENT} <!-- Dynamic content goes here -->
      </div>
    </div>

    <!-- Right Column for View Container (iframe) -->
    <div class="right-column">
      <div class="view-container">
        <iframe name="view" width="100%" height="100%"></iframe>
      </div>
    </div>
  </div>
</body>
</html>
"#;
