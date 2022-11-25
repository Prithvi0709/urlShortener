// Wait for the DOM to load
document.addEventListener('DOMContentLoaded', (event) => {
            var wage = document.getElementById("url_shtrn");
            wage.addEventListener("keydown", function (e) {
                if (e.code === "Enter") {  //checks whether the pressed key is "Enter"
                    getCheckedRadio(); // Checks if a radio button is chosen
                    fetch_shorten();
                }
            });

            function fetch_shorten() {
                var url = document.getElementById("url_shtrn").value;

                // Frontend checks for url integrity.
                if (url == "") {
                    alert("Please enter a valid URL");
                    return;
                }   
                if (!url.includes(".")) {
                    alert("Please enter a valid URL");
                    return;
                }
                
                
                var checkedOption = getCheckedRadio();
                fetch('/shorten?url='+document.getElementById('url_shtrn').value+"&translation_type="+checkedOption)
                .then((response) => response.json())
                .then((data) => {
                    var j = getCurrentURL() + data;
                    document.getElementById('final_url').href = j;
                    document.getElementById('final_url').innerHTML = j;
                    document.getElementById('url_disp_p').style.display = "block";
                    
                });
                
            }

        })

        function getCheckedRadio() {
                var ele = document.getElementsByName('shorten_type');
                for(i = 0; i < ele.length; i++) {
                    if(ele[i].checked){
                        return ele[i].getAttribute('id');
                        break;
                    }
                }
                alert("Please choose a method to shorten");    
        }

        function getCurrentURL () {
            return window.location.href
        }
