function ASCII() {

	var plaintext = document.querySelector("#text").value;
	var ascii = "";

	for (i = 0; i<plaintext.length; i++){
		ascii += " " + plaintext.charCodeAt(i);
	}
	document.querySelector("#ascii").innerHTML = ascii;

}
function AsciiToPlain(){
	var ascii = document.querySelector("#ascii").value;
	var plain = "";

}

