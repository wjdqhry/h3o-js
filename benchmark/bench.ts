import {Bench} from 'tinybench';
import {latLngToCell, cellToLatLng} from '../index.js';
import H3 from 'h3-js';

const latlngToCellBench = new Bench();
const cellToLatlngBench = new Bench();

latlngToCellBench.add('latlngToCell-H3o', () => {
    latLngToCell(37.504521, 127.088092, 9);
});
latlngToCellBench.add('latlngToCell-H3', () => {
    H3.latLngToCell(37.504521, 127.088092, 9);
});

cellToLatlngBench.add('cellToLatlng-H3o', () => {
    cellToLatLng('8930e1caaa7ffff');
});
cellToLatlngBench.add('cellToLatlng-H3', () => {
    H3.cellToLatLng('8930e1caaa7ffff');
})

await latlngToCellBench.run();
await cellToLatlngBench.run();

console.table(latlngToCellBench.table());
console.log('---------------------------------');
console.table(cellToLatlngBench.table());