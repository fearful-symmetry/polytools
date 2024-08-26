use std::collections::HashMap;



/// a basic template for sending alerts to a device
const ALERT_TMPL: &str =  r#"
<head>
    <style>
        body{background-color:black}
        .container{position:absolute;left:50%;top:50%;margin:-80px 0 0 -140px;}
        .box{background: #ff0909;border-radius: 0px 0px 0px 0px;width: 280px;max-height: 150px;word-wrap: break-word;overflow: hidden;border: 1px solid #808080;margin: 0px auto;}
        .box bold{font-weight:bold;font-family : geneva, helvetica;color : #FFFFFF; font-size : medium;}
        .box p{ font-family : geneva, helvetica;color : #FFFFFF; font-size : small;margin:10px 10px 25px 10px;}
    </style>
</head>
<body>
    <div class="container">
        <div class="box">
            <p>
                <bold>{{title}}</bold><br>
                {{body}}<br>
            </p>
        </div>
    </div>
</body>
"#;

pub fn render_alert_template(header: String, body: String) -> anyhow::Result<String> {
    let mut hlbrs = handlebars::Handlebars::new();
    hlbrs.register_template_string("base", ALERT_TMPL)?;
    let  data: HashMap<&str, &str> = HashMap::from([("title", header.as_ref()), ("body", body.as_ref())]);
    let res = hlbrs.render("base", &data)?;
    Ok(res)
}