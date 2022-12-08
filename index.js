function say(input) {
  console.log(input)
  //damit ich bei console.log nicht immer so viel schreiben muss
}

function copy(a) {
    var copyText = a;
    navigator.clipboard.writeText(copyText).then(() => {
      notiv('copied '+a+' to clipboard')
})};

function farbe(val) {
  document.documentElement.style.setProperty("--base-1", val);
  //nutze den Input "val" aus dem onclick event im HTML um die grundfarbe zu setzen
  save(val, 'theme')
  //schreibe die Änderungen zum localstorage
  if (document.querySelector('#base-hsl-slider')) {
    document.querySelector('#base-hsl-slider').value = val
  }
}

function save(color, type) {
  //unterscheide zwischen Grundfarbe und highlight farbe
  if (type == 'theme') {
    window.localStorage.setItem("color", color)
    //schreibe zu localstorage   
  } else if (type == 'high') {
    window.localStorage.setItem('high', color)
  } else if (type == 'cont') {
    window.localStorage.setItem('cont', color)
  }
}

function init() {
  //erstmaliges laden beim Laden der Seite
  let local = window.localStorage.getItem('color')
  let localhigh = window.localStorage.getItem('high')
  let localcont = window.localStorage.getItem('cont')
  if (local == null) {
    farbe(209)
    high(28)
    cont(100)
  } else {
    farbe(local)
    high(localhigh)
    cont(localcont)
  }
  //setzen der Beiden farben
  scroll_p()
  //setzen des "x/3" beim Slider-Element
  window.setTimeout(initnotiv, 3000)
}

function initnotiv() {
  notiv('Welcome to my corner of the internet! <br>' + 
  'If you have any suggestions or issues please contact me')
}

function high(val) {
  //wie bei der Grundfarbe auch
  document.documentElement.style.setProperty("--high-1", val)
  save(val, 'high')
  if (document.querySelector('#high-hsl-slider')) {
  document.querySelector('#high-hsl-slider').value = val
  }
}

function cont(val) {
  document.documentElement.style.setProperty("--cont-1", val)
  save(val , 'cont')
  if (document.querySelector('#cont-hsl-slider')) {
    document.querySelector('#cont-hsl-slider').value = val
  }
}

let sel_more_scroller = 1
//die aktive Slide des scroller Elementes
function more_move(richtung) {
  let mehr = document.getElementById('mehrw')
  // wenn der linke pfeil geclickt wird, und die ausgewählte silde nicht die erste ist
  if (richtung === 1 && sel_more_scroller > 1) {
    sel_more_scroller = sel_more_scroller - 1
    //dann gehe eine nach links -> subtrahiere 
    mehr.style.setProperty('--selected', sel_more_scroller)
    //property setzten um es auf das CSS anzuwenden
    scroll_p()
    //Zähler aktualisieren
    //wenn nach rechts & nicht die letzte slide
  } else if (richtung === 2 && sel_more_scroller < 2) {
    //eins nach rechts -> addiren
    sel_more_scroller = sel_more_scroller + 1
    //anwenden
    mehr.style.setProperty('--selected', sel_more_scroller)
    //zähler aktualisieren
    scroll_p()
  }
}

function scroll_p() {
  //die momentan ausgewählte slide im Zähler anzeigen
  if (document.querySelector('.scroll_progress')) {
    document.querySelector('.scroll_progress').innerHTML = sel_more_scroller + "/2"
  }
}

function img(el, msg) {
  var imgSrc = el.src;
  say(imgSrc)
  let div = document.createElement("div");
  div.className = "img-popup";
  if (msg == undefined) {
    div.innerHTML = "<div class='img-popupw'><div class='close-popup' onclick='closeimg()'>X</div><img src='"+imgSrc+"'></div>"
  } else {
    div.innerHTML = "<div class='img-popupw'><div class='close-popup' onclick='closeimg()'>X</div><img src='"+imgSrc+"'><div class='popup-content'>"+msg+"</div></div>"
  }
  document.body.appendChild(div);
}

function closeimg() {
  let img = document.querySelector('.img-popup')
  img.parentNode.removeChild(img)
}

function baseHSL() {
  let val = document.querySelector('#base-hsl-slider').value
  farbe(val)
}

function highHSL() {
  let val = document.querySelector('#high-hsl-slider').value
  high(val)
}

function contHSL() {
  let val = document.querySelector('#cont-hsl-slider').value
  cont(val)
}

function notiv(content) {
  let div = document.createElement("div");
  div.className = "notiv";
  div.innerHTML = content
  document.body.appendChild(div);
  let el = document.querySelector('.notiv')
  window.setTimeout(notivin, 10)
}

function notivin() {
  let el = document.querySelector('.notiv')
  el.classList.add('notiv-anim-in')
  window.setTimeout(notivout, 6000)
}

function notivout() {
  let el = document.querySelector('.notiv')
  el.classList.remove('notiv-anim-in')
  el.classList.add('notiv-anim-out')
  window.setTimeout(notiv_del, 2000)
}

function notiv_del() {
  let el = document.querySelector('.notiv')
  el.parentNode.removeChild(el)
}
