import * as FileLoader from "file-loader";
window.change = function(event) {
  var file = event.target.files[0];
  var reader = new FileReader();
  reader.onload = function(event) {
    // console.log(event.target.result)
    console.log(FileLoader.pdf_check(event.target.result.split('base64,')[1]))
  };

  if (file) reader.readAsDataURL(file);
}
