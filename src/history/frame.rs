//history frame - Это кадр истории, который отображает все тикеры на данный момент


struct SecurityFrame{
    security: &str,
    canle: Candle,
}


struct HistoryFrame
{
    dt: DateTime<Local>,
    securities: Vec<SecurityFrame>,
}