pub const htmlStartDefine: &str = r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Document</title>
</head>

<body>
    <div class="wrap">
<!--         <input type="button" value="新增" onclick="createRow()">
        <input type="button" value="删除" onclick="delRow()">
 -->
        <table id="table" border="1" width="800" cellspacing=0 cellpadding=0>
            <tr>
                <th>state</th>
                <th>description</th>
                <th>name</th>
                <th>action</th>
            </tr>
            <tbody id="tbody">

            </tbody>
        </table>
    </div>
</body>
<script type="text/javascript">

    var obj = {
        state:'runnning',
        description:'description',
        name:'name'
    }
    function create(obj) {
        var tab=document.getElementById("table");
        id = tab.rows.length;
        var editTable = document.getElementById("tbody");
        var tr = document.createElement("tr");
        var td0 = document.createElement("td");
        td0.innerHTML = obj.state;
        var td1 = document.createElement("td");
        td1.innerHTML = obj.description;
        var td2 = document.createElement("td");
        td2.innerHTML = obj.name;
        var td3 = document.createElement("td");
        td3.innerHTML = "<button id='rs"+id+"' onclick='restart("+id+")'>restart</button><button id='st"+id+"' onclick='stop("+id+")'>stop</button>";
        tr.appendChild(td0);
        tr.appendChild(td1);
        tr.appendChild(td2);
        tr.appendChild(td3);
        editTable.appendChild(tr);
    }

    //重启
    function restart(id){
       
        var tab=document.getElementById("table");        
        tab.rows[id].children[0].innerHTML = 'start';
    }
    //停止
    function stop(id){
        var tab=document.getElementById("table");        
        tab.rows[id].children[0].innerHTML = 'stop';
    }

    function createRow(){
        create(obj);
    }


    function getData(){
    }
"#;

pub const htmlEndDefine: &str = r#"
</script>

</html>
"#;