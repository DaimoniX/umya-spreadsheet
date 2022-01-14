// c:scatterChart
use super::ScatterStyle;
use super::VaryColors;
use super::AreaChartSeries;
use super::DataLabels;
use super::AxisId;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ScatterChart {
    scatter_style: ScatterStyle,
    vary_colors: VaryColors,
    area_chart_series: Vec<AreaChartSeries>,
    data_labels: DataLabels,
    axis_id: Vec<AxisId>,
}
impl ScatterChart {
    pub fn get_scatter_style(&self)-> &ScatterStyle {
        &self.scatter_style
    }

    pub fn get_scatter_style_mut(&mut self)-> &mut ScatterStyle {
        &mut self.scatter_style
    }

    pub fn set_scatter_style(&mut self, value:ScatterStyle)-> &mut ScatterChart {
        self.scatter_style = value;
        self
    }

    pub fn get_vary_colors(&self)-> &VaryColors {
        &self.vary_colors
    }

    pub fn get_vary_colors_mut(&mut self)-> &mut VaryColors {
        &mut self.vary_colors
    }

    pub fn set_vary_colors(&mut self, value:VaryColors)-> &mut ScatterChart {
        self.vary_colors = value;
        self
    }

    pub fn get_area_chart_series(&self)-> &Vec<AreaChartSeries> {
        &self.area_chart_series
    }

    pub fn get_area_chart_series_mut(&mut self)-> &mut Vec<AreaChartSeries> {
        &mut self.area_chart_series
    }

    pub fn set_area_chart_series(&mut self, value:Vec<AreaChartSeries>)-> &mut ScatterChart {
        self.area_chart_series = value;
        self
    }

    pub fn add_area_chart_series(&mut self, value:AreaChartSeries)-> &mut ScatterChart {
        self.area_chart_series.push(value);
        self
    }

    pub fn get_data_labels(&self)-> &DataLabels {
        &self.data_labels
    }

    pub fn get_data_labels_mut(&mut self)-> &mut DataLabels {
        &mut self.data_labels
    }

    pub fn set_data_labels(&mut self, value:DataLabels)-> &mut ScatterChart {
        self.data_labels = value;
        self
    }

    pub fn get_axis_id(&self)-> &Vec<AxisId> {
        &self.axis_id
    }

    pub fn get_axis_id_mut(&mut self)-> &mut Vec<AxisId> {
        &mut self.axis_id
    }

    pub fn set_axis_id(&mut self, value:Vec<AxisId>)-> &mut ScatterChart {
        self.axis_id = value;
        self
    }

    pub fn add_axis_id(&mut self, value:AxisId)-> &mut ScatterChart {
        self.axis_id.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:ser" => {
                            let mut obj = AreaChartSeries::default();
                            obj.set_attributes(reader, e);
                            self.add_area_chart_series(obj);
                        },
                        b"c:dLbls" => {
                            self.data_labels.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:scatterStyle" => {
                            self.scatter_style.set_attributes(reader, e);
                        },
                        b"c:varyColors" => {
                            self.vary_colors.set_attributes(reader, e);
                        },
                        b"c:axId" => {
                            let mut obj = AxisId::default();
                            obj.set_attributes(reader, e);
                            self.add_axis_id(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:scatterChart" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:scatterChart"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:scatterChart
        write_start_tag(writer, "c:scatterChart", vec![], false);

        // c:scatterStyle
        &self.scatter_style.write_to(writer);

        // c:varyColors
        &self.vary_colors.write_to(writer);

        // c:ser
        for v in &self.area_chart_series {
            v.write_to(writer);
        }

        // c:dLbls
        &self.data_labels.write_to(writer);

        // c:axId
        for v in &self.axis_id {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:scatterChart");
    }
}
