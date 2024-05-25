function showMap(lon, lat, zoom) {
    //console.log("showMap");
    const myMap = L.map("myMap");
    const tiles = L.tileLayer(
        'https://tile.openstreetmap.de/{z}/{x}/{y}.png',
        {
            maxZoom: 19,
            attribution: "© Leaflet, <a href='https://www.openstreetmap.org'>OSM</a>"
        }
    )
    tiles.addTo(myMap);
    myMap.attributionControl.setPrefix(false);
    myMap.setView([lat, lon], zoom);
}