import * as FileLoader from "file-loader";
window.change = function(event) {
  var file = event.target.files[0];
  var reader = new FileReader();
  reader.onload = function(event) {
    let base64Str = event.target.result.split('data:application/pdf;base64,')[1]
    console.log(FileLoader.pdf_check(base64Str))
  };

  if (file) reader.readAsDataURL(file);
}
