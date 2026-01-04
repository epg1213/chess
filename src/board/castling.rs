use crate::board::pieces::*;
use std::sync::Arc;
use crate::board::Board;
use crate::board::BoardError;

impl Board {
    pub fn remove_castling_rights(&mut self, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) {
        let from = from_sq_name.as_ref();
        let to = to_sq_name.as_ref();
        if from=="E1" || from=="H1" || to=="E1" || to=="H1" {
            self.white_short_allowed = false;
        }
        if from=="E1" || from=="A1" || to=="E1" || to=="A1" {
            self.white_long_allowed = false;
        }
        if from=="E8" || from=="H8" || to=="E8" || to=="H8" {
            self.black_short_allowed = false;
        }
        if from=="E8" || from=="A8" || to=="E8" || to=="A8" {
            self.black_long_allowed = false;
        }
    }
    pub fn try_castle(&mut self, to_sq_name: impl AsRef<str>, piece: Arc<dyn ChessPiece>) -> Result<bool, BoardError> {
        if piece.clone().is_king() && piece.clone().color() == Color::White && to_sq_name.as_ref()=="G1" {
            return Ok(self.white_short_castle()?);
        }
        if piece.clone().is_king() && piece.clone().color() == Color::White && to_sq_name.as_ref()=="C1" {
            return Ok(self.white_long_castle()?);
        }
        if piece.clone().is_king() && piece.clone().color() == Color::Black && to_sq_name.as_ref()=="G8" {
            return Ok(self.black_short_castle()?);
        }
        if piece.clone().is_king() && piece.clone().color() == Color::Black && to_sq_name.as_ref()=="C8" {
            return Ok(self.black_long_castle()?);
        }
        Ok(false)
    }
    fn white_short_castle(&mut self) -> Result<bool, BoardError> {
        if !self.white_short_allowed ||
            !self.square_as_ref("E1")?.piece.clone().is_some_and(|p| p.is_king()&&p.color()==Color::White) ||
            !self.square_as_ref("H1")?.piece.clone().is_some_and(|p| p.is_rook()&&p.color()==Color::White) ||
            self.square_as_ref("F1")?.piece.is_some() ||
            self.square_as_ref("G1")?.piece.is_some() ||
            self.controls(Color::Black, "F1") ||
            self.controls(Color::Black, "G1") ||
            self.controls(Color::Black, "E1") {
            return Ok(false)
        }
        self.put("E1", None)?;
        self.put("G1", Some(Arc::new(king::King::white())))?;
        self.put("H1", None)?;
        self.put("F1", Some(Arc::new(rook::Rook::white())))?;
        self.white_short_allowed = false;
        self.white_long_allowed = false;
        Ok(true)
    }
    fn white_long_castle(&mut self) -> Result<bool, BoardError> {
        if !self.white_long_allowed ||
            !self.square_as_ref("E1")?.piece.clone().is_some_and(|p| p.is_king()&&p.color()==Color::White) ||
            !self.square_as_ref("A1")?.piece.clone().is_some_and(|p| p.is_rook()&&p.color()==Color::White) ||
            self.square_as_ref("B1")?.piece.is_some() ||
            self.square_as_ref("C1")?.piece.is_some() ||
            self.square_as_ref("D1")?.piece.is_some() ||
            self.controls(Color::Black, "C1") ||
            self.controls(Color::Black, "D1") ||
            self.controls(Color::Black, "E1") {
            return Ok(false)
        }
        self.put("E1", None)?;
        self.put("C1", Some(Arc::new(king::King::white())))?;
        self.put("A1", None)?;
        self.put("D1", Some(Arc::new(rook::Rook::white())))?;
        self.white_short_allowed = false;
        self.white_long_allowed = false;
        Ok(true)
    }
    fn black_short_castle(&mut self) -> Result<bool, BoardError> {
        if !self.black_short_allowed ||
            !self.square_as_ref("E8")?.piece.clone().is_some_and(|p| p.is_king()&&p.color()==Color::Black) ||
            !self.square_as_ref("H8")?.piece.clone().is_some_and(|p| p.is_rook()&&p.color()==Color::Black) ||
            self.square_as_ref("F8")?.piece.is_some() ||
            self.square_as_ref("G8")?.piece.is_some() ||
            self.controls(Color::White, "F8") ||
            self.controls(Color::White, "G8") ||
            self.controls(Color::White, "E8") {
            return Ok(false)
        }
        self.put("E8", None)?;
        self.put("G8", Some(Arc::new(king::King::black())))?;
        self.put("H8", None)?;
        self.put("F8", Some(Arc::new(rook::Rook::black())))?;
        self.black_short_allowed = false;
        self.black_long_allowed = false;
        Ok(true)
    }
    fn black_long_castle(&mut self) -> Result<bool, BoardError> {
        if !self.black_long_allowed ||
            !self.square_as_ref("E8")?.piece.clone().is_some_and(|p| p.is_king()&&p.color()==Color::Black) ||
            !self.square_as_ref("A8")?.piece.clone().is_some_and(|p| p.is_rook()&&p.color()==Color::Black) ||
            self.square_as_ref("B8")?.piece.is_some() ||
            self.square_as_ref("C8")?.piece.is_some() ||
            self.square_as_ref("D8")?.piece.is_some() ||
            self.controls(Color::White, "C8") ||
            self.controls(Color::White, "D8") ||
            self.controls(Color::White, "E8") {
            return Ok(false)
        }
        self.put("E8", None)?;
        self.put("C8", Some(Arc::new(king::King::black())))?;
        self.put("A8", None)?;
        self.put("D8", Some(Arc::new(rook::Rook::black())))?;
        self.black_short_allowed = false;
        self.black_long_allowed = false;
        Ok(true)
    }
}

