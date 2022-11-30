// Wait for the DOM to load
document.addEventListener('DOMContentLoaded', (event) => {
            var wage = document.getElementById("url_shtrn");
            wage.addEventListener("keydown", function (e) {
                if (e.code === "Enter") {  //checks whether the pressed key is "Enter"
                    getCheckedRadio(); // Checks if a radio button is chosen
                    fetch_shorten();
                }
            });

            var tracker = document.getElementById("url_track");
            wage.addEventListener("keydown", function (e) {
                if (e.code === "Enter") {  //checks whether the pressed key is "Enter"
                    track();
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
                .then((response) => response.text())
                .then((data) => {
                    var j = getCurrentURL() + data;
                    document.getElementById('final_url').href = j;
                    document.getElementById('final_url').innerHTML = j;
                    document.getElementById('url_disp_p').style.display = "block";
                });
                
            }

            function track() {

                var url = document.getElementById("url_track").value;

                // Frontend checks for url integrity.
                if (url == "") {
                    alert("Please enter a valid URL");
                    return;
                }   
                if (!url.includes(".")  && !url.includes("/") ) {
                    alert("Please enter a valid URL");
                    return;
                } 
                
                var extracted_key = keyExtraction(url);

                fetch('/track?<hkey>='+extracted_key)
                .then((response) => response.text())
                .then((data) => {
                    var track_info = data.split('prithviandsamadandkamal')
                    
                    document.getElementById('count').innerHTML = track_info[1]
                    document.getElementById('web_name').innerHTML = track_info[0] 

                    document.getElementById("count_display_div").style.display = "block";
                })


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

        function keyExtraction(url){
            const array = url.split('/')
            var key = array[array.length()-1]
            return key
        }
