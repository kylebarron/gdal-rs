use std::ffi::CString;
use vector::Layer;
use utils::_string;
use vector::ogr;
use vector::geometry::Geometry;


pub struct Feature<'a> {
    _layer: &'a Layer<'a>,
    c_feature: *const (),
    geometry: Geometry,
}


impl<'a> Feature<'a> {
    pub unsafe fn _with_layer(layer: &'a Layer<'a>, c_feature: *const ()) -> Feature<'a> {
        return Feature{
            _layer: layer,
            c_feature: c_feature,
            geometry: Geometry::lazy_feature_geometry(),
        };
    }

    pub fn field(&self, name: &str) -> Option<FieldValue> {
        let c_name = CString::new(name.as_bytes()).unwrap();
        let field_id = unsafe { ogr::OGR_F_GetFieldIndex(self.c_feature, c_name.as_ptr()) };
        if field_id == -1 {
            return None;
        }
        let field_defn = unsafe { ogr::OGR_F_GetFieldDefnRef(self.c_feature, field_id) };
        let field_type = unsafe { ogr::OGR_Fld_GetType(field_defn) };
        return match field_type {
            ogr::OFT_STRING => {
                let rv = unsafe { ogr::OGR_F_GetFieldAsString(self.c_feature, field_id) };
                return Some(FieldValue::StringValue(_string(rv)));
            },
            ogr::OFT_REAL => {
                let rv = unsafe { ogr::OGR_F_GetFieldAsDouble(self.c_feature, field_id) };
                return Some(FieldValue::RealValue(rv as f64));
            },
            _ => panic!("Unknown field type {}", field_type)
        }
    }

    pub fn geometry(&self) -> &Geometry {
        if ! self.geometry.has_gdal_ptr() {
            let c_geom = unsafe { ogr::OGR_F_GetGeometryRef(self.c_feature) };
            unsafe { self.geometry.set_c_geometry(c_geom) };
        }
        return &self.geometry;
    }

    pub fn wkt(&self) -> String {
        return self.geometry().wkt();
    }


    pub fn json(&self) -> String {
        return self.geometry().json();
    }
}


impl<'a> Drop for Feature<'a> {
    fn drop(&mut self) {
        unsafe { ogr::OGR_F_Destroy(self.c_feature); }
    }
}


pub enum FieldValue {
    StringValue(String),
    RealValue(f64),
}


impl FieldValue {
    pub fn as_string(self) -> String {
        match self {
            FieldValue::StringValue(rv) => rv,
            _ => panic!("not a StringValue")
        }
    }

    pub fn as_real(self) -> f64 {
        match self {
            FieldValue::RealValue(rv) => rv,
            _ => panic!("not a RealValue")
        }
    }
}
