<script>
  const inputCheck = () => {
    var inputValue = document.getElementById( "textbox" ).value;
    document.getElementById( "list" ).innerHTML = '';
    let list = '';
    if(inputValue !== '') {
      list += '<div style="border-top: solid 1px #AAAAAA;"></div>';
      ['1', '2', inputValue].forEach((value)=>{
        list += `<div class="line" style="margin-left: 10px;padding: 0.4em;cursor: pointer;" onclick="selectValue()">${value}</div>`;
      });
    }
    document.getElementById( "list" ).innerHTML = list;
  };
</script>

<main>
  <p>input:</p><br>
  <div id="search">
    <div id="input">
      <input id="textbox" class="line" type="text" on:keyup={inputCheck}>
      <div id="list"></div>
    </div>
  </div>
  <p>重なり</p>
  <script type="text/javascript">
    function selectValue() {
      document.getElementById( "textbox" ).value = event.target.innerText;
      document.getElementById( "list" ).innerHTML = '';
    };
  </script>
</main>

<style>
#search {
  height: 20px;
}
#input {
  margin: 5px;
  border-radius: 10px;
  border: solid 1px #AAAAAA;
  position: absolute;
  z-index: 1;
  background-color: #FFFFFF;
}
.line {
  padding: 0.4em;
  margin-right: 10px;
  margin-left: 10px;
  margin-bottom: 0px;
  margin-top: 0px;
}
input[type='text'] {
	border: none;
}
input[type='text']:focus {
	outline: none;
}
</style>
