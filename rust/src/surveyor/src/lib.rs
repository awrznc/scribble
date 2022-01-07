pub struct Writer<'a> {
    path: &'a str,
    label: &'a str,
    shp_dest: std::io::Cursor<Vec::<u8>>,
    shx_dest: std::io::Cursor<Vec::<u8>>,
    dbf_dest: std::io::Cursor<Vec::<u8>>,
    geojson_dest: Vec::<u8>
}

impl<'a> Writer<'a> {
    pub fn new( path: &'a str, label: &'a str ) -> Writer<'a> {
        Writer {
            path: path,
            label: label,
            shp_dest: std::io::Cursor::new(Vec::<u8>::new()),
            shx_dest: std::io::Cursor::new(Vec::<u8>::new()),
            dbf_dest: std::io::Cursor::new(Vec::<u8>::new()),
            geojson_dest: Vec::<u8>::new(),
        }
    }

    pub fn write_all( mut self ) -> Result<(), Box<dyn std::error::Error>> {

        // Generate Shapefile
        {
            // Shapefile
            let ring = shapefile::PolygonRing::Outer(vec![
                shapefile::PointZ::new(0.0, 0.0, 0.0, shapefile::NO_DATA),
                shapefile::PointZ::new(0.2, 1.2, 0.0, shapefile::NO_DATA),
                shapefile::PointZ::new(1.0, 1.0, 0.0, shapefile::NO_DATA),
                shapefile::PointZ::new(1.0, 0.0, 0.0, shapefile::NO_DATA),
            ]);
            let polygon = shapefile::PolygonZ::new(ring);

            // dbase
            let mut record = shapefile::dbase::Record::default();
            record.insert("label".to_string(), shapefile::dbase::FieldValue::Character(Some("Hogehoge".to_string())));

            // write
            let shape_writer = shapefile::ShapeWriter::with_shx(&mut self.shp_dest, &mut self.shx_dest);
            let dbase_writer = shapefile::dbase::TableWriterBuilder::new()
                .add_character_field(shapefile::dbase::FieldName::try_from("label")?, 50)
                .build_with_dest(&mut self.dbf_dest);
            let mut writer = shapefile::Writer::new(shape_writer, dbase_writer);
            writer.write_shape_and_record(&polygon, &record)?;
        }

        // Shapefile (shp + shx + dbf) to GeoJSON
        {
            // reset seek position.
            use std::io::Seek;
            self.shp_dest.seek(std::io::SeekFrom::Start(0))?;
            self.shx_dest.seek(std::io::SeekFrom::Start(0))?;
            self.dbf_dest.seek(std::io::SeekFrom::Start(0))?;

            // write
            use geozero::geojson::GeoJsonWriter;
            let mut reader = geozero_shp::Reader::new(&mut self.shp_dest)?;
            reader.add_index_source(&mut self.shx_dest)?;
            reader.add_dbf_source(&mut self.dbf_dest)?;
            reader.iter_features(GeoJsonWriter::new(&mut self.geojson_dest))?.count();
        }

        // export
        {
            use std::io::Write;
            std::fs::File::create(self.path.to_string() + self.label + ".shp")?.write(&self.shp_dest.get_ref())?;
            std::fs::File::create(self.path.to_string() + self.label + ".shx")?.write(&self.shx_dest.get_ref())?;
            std::fs::File::create(self.path.to_string() + self.label + ".dbf")?.write(&self.dbf_dest.get_ref())?;
            std::fs::File::create(self.path.to_string() + self.label + ".geojson")?.write(&self.geojson_dest)?;
        }

        Ok(())
    }
}
