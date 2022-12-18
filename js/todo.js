function post() {
    let input = document.getElementById("todo-input")
    let out = document.querySelector(".out")
    let li = document.createElement("li");
    li.innerHTML = input.value + '<div class="cl" onclick="cl()">X</div>'
    // let cl = document.createElement("div");
    // cl.classList.add('cl')
    // cl.innerHTML = "X"
    out.appendChild(li)
    // li.appendChild(cl)
    savepost()
    input.value = ''
}
function savepost() {

}
function cl() {
    
}