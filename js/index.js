function say(input) {
  console.log(input)
  //damit ich bei console.log nicht immer so viel schreiben muss
}

function copy(a) {
    var copyText = a;
    //clipboard api with passthrough var (html)
    navigator.clipboard.writeText(copyText).then(() => {
      notiv('copied '+a+' to clipboard')
})};

function farbe(val) {
  document.documentElement.style.setProperty("--base-1", val);
  //nutze den Input "val" aus dem onclick event im HTML um die grundfarbe zu setzen
  save(val, 'theme')
  //schreibe die Änderungen zum localstorage
  //check if element is existing (don't try to perform action without element)
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
    //schreibe zu localstorage   
  } else if (type == 'cont') {
    //schreibe zu localstorage   
    window.localStorage.setItem('cont', color)
  }
}

function init() {
  //erstmaliges laden beim Laden der Seite
  let local = window.localStorage.getItem('color')
  let localhigh = window.localStorage.getItem('high')
  let localcont = window.localStorage.getItem('cont')
  if (local == null) {
    //wenn keine gespeicherten wert -> setze defaults
    farbe(209)
    high(28)
    cont(100)
  } else {
    //sonst nutze die localstorage items
    farbe(local)
    high(localhigh)
    cont(localcont)
  }
  //setzen der drei farben
  scroll_p()
  //setzen des "x/3" beim Slider-Element
}

function high(val) {
  //wie bei der Grundfarbe auch
  document.documentElement.style.setProperty("--high-1", val)
  save(val, 'high')
  //siehe oben
  if (document.querySelector('#high-hsl-slider')) {
  document.querySelector('#high-hsl-slider').value = val
  }
}

function cont(val) {
  //siehe oben, color()
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
  //quelle des bildes aus dem ersten Parameter fetchen
  var imgSrc = el.src;
  //das popup erschaffen
  let div = document.createElement("div");
  div.className = "img-popup";
  //wenn keine msg, dann kein content element
  if (msg == undefined) {
    div.innerHTML = "<div class='img-popupw'><div class='close-popup' onclick='closeimg()'>X</div><img src='"+imgSrc+"'></div>"
  } else {
    //sonst schon
    div.innerHTML = "<div class='img-popupw'><div class='close-popup' onclick='closeimg()'>X</div><img src='"+imgSrc+"'><div class='popup-content'>"+msg+"</div></div>"
  }
  document.body.appendChild(div);
}

function closeimg() {
  //kill element on click
  let img = document.querySelector('.img-popup')
  img.parentNode.removeChild(img)
}

function baseHSL() {
  //das slider element des config guis dem wert anpassen (wenn der knopf geclickt wird/ init)
  let val = document.querySelector('#base-hsl-slider').value
  farbe(val)
}

function highHSL() {
  //oben
  let val = document.querySelector('#high-hsl-slider').value
  high(val)
}

function contHSL() {
  //oben
  let val = document.querySelector('#cont-hsl-slider').value
  cont(val)
}

function notiv(content) {
  //notiv div erschaffen
  let div = document.createElement("div");
  div.className = "notiv";
  div.innerHTML = content
  //inhalt einfügen
  document.body.appendChild(div);
  let el = document.querySelector('.notiv')
  //notivanimations funktion starten
  window.setTimeout(notivin, 10)
}
/*
           _
          |E]
        .-|=====-.
        | | mail |
     ___|________|
            ||
            ||
            ||   www
     ,;,    ||   )_(,;;;,
     <_>  \ ||   \|/ \_/
     \|/  \\||  \\|   |//
_jgs_\|//_\\|///_\V/_\|//__
*/
function notivin() {
  //animation in
  let el = document.querySelector('.notiv')
  //classe hinzufügen
  el.classList.add('notiv-anim-in')
  //nach timeout anim out
  window.setTimeout(notivout, 6000)
}

function notivout() {
  let el = document.querySelector('.notiv')
  //classen "tauschen"
  el.classList.remove('notiv-anim-in')
  el.classList.add('notiv-anim-out')
  //notiv element löschen, nach timeout
  window.setTimeout(notiv_del, 2000)
}

function notiv_del() {
  //löschen des notiv elements
  let el = document.querySelector('.notiv')
  el.parentNode.removeChild(el)
}
