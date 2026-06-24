use ratatui::{
    layout::{Alignment}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph, Widget}
};

const SIGN_MASK: u64 =     0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const EXPONENT_MASK: u64 = 0b0111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const OFFSET_MASK: u64 =   0b1000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

#[derive(Debug, Clone, Copy)]
pub enum DisplayBase {
    Hex,
    Binary,
}

pub struct FloatComponents {
    num: f64,
    bits: u64,
    sign: u64,
    exponent: u64,
    offset: u64,
    base: DisplayBase,
}

impl FloatComponents {
    pub fn new(n: f64, b: DisplayBase) -> Self {
        let bits: u64 = n.to_bits();
        FloatComponents {
            num: n,
            bits,
            sign: (bits & SIGN_MASK) >> 63,
            exponent: (bits & EXPONENT_MASK) >> 52,
            offset: bits & OFFSET_MASK,
            base: b,
        }
    }

    fn format(&self) -> String {
        match self.base {
            DisplayBase::Binary => format!(
            "value:    {4:e}\n\
                bits:   {0:#064b}\n\
                sign:     {1} ({1})\n\
                exponent: {2:#011b}\n\
                offset:   {3:#0b} ({3})",
            self.bits, self.sign, self.exponent, self.offset, self.num),
            DisplayBase::Hex => format!(
            "value:    {4:e}\n\
                bits:   {0:#016}\n\
                sign:     {1} ({1})\n\
                exponent: {2:#03x}\n\
                offset:   {3:#0x} ({3})",
            self.bits, self.sign, self.exponent, self.offset, self.num)

        }
    }
}

impl Widget for FloatComponents {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized
    {
        
        Paragraph::new(self.format())
        .block(
            Block::default()
                .title(" Values ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left)
        .render(area, buf);
    }
}

    /*
    Rendering and such
    let val: f64 = 10.87;
    let fbits: u64 = val.to_bits();
    let r: FloatComponents = deconstruct(val);
    */
