use std::str::FromStr;
use h3o::CellIndex;
use napi_derive::napi;

#[napi]
pub fn latLngToCell(lat: f64, lng: f64, resolution: u8) -> String {
    h3o::LatLng::new(lat, lng)
        .unwrap()
        .to_cell(h3o::Resolution::try_from(resolution).unwrap())
        .to_string()
}

#[napi]
pub fn cellToLatLng(cell: String) -> Option<Vec<f64>> {
    CellIndex::from_str(&cell).ok().map(|cell_index| {
        let latlng = h3o::LatLng::from(cell_index);
        vec![latlng.lat(), latlng.lng()]
    })
}

#[napi]
pub fn cellToBoundary(cell: String, geojson: bool) -> Option<Vec<Vec<f64>>> {
    CellIndex::from_str(&cell).ok().map(|cell_index| {
        cell_index.boundary().iter().map(|ll|
            return if geojson {
                vec![ll.lng(), ll.lat()]
            } else {
                vec![ll.lat(), ll.lng()]
            }
        ).collect()
    })
}

#[napi(object)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lat_lng_to_cell_test() {
        assert_eq!(latLngToCell(37.504521, 127.088092, 9), "8930e1caaa7ffff");
    }

    #[test]
    fn cell_to_lat_lng_test() {
        let value = cellToLatLng("8930e1caaa7ffff".to_string()).unwrap();
        assert_eq!(value.len(), 2);
        assert_eq!(value[0], 37.504074214636084);
        assert_eq!(value[1], 127.08911501704418);
    }
}