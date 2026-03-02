//! Trading history query tool - view your trading database
//!
//! Usage: cargo run --bin trading_history [command] [options]

use anyhow::Result;
use prediction_market_arbitrage::database::{TradingDatabase, print_recent_activity, print_today_summary};
use rusqlite::params;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    let db = TradingDatabase::new()?;
    
    if args.len() < 2 {
        print_help();
        print_today_summary(&db)?;
        print_recent_activity(&db, 10)?;
        return Ok(());
    }
    
    match args[1].as_str() {
        "summary" => {
            print_today_summary(&db)?;
        }
        "recent" => {
            let limit: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
            print_recent_activity(&db, limit)?;
        }
        "stats" => {
            print_all_time_stats(&db)?;
        }
        "opportunities" => {
            let limit: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
            print_recent_opportunities(&db, limit)?;
        }
        "daily" => {
            let days: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(7);
            print_daily_summary(&db, days)?;
        }
        "export" => {
            export_to_csv(&db)?;
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("\n Trading History Query Tool");
    println!("{:=<80}", "");
    println!("\nCommands:");
    println!("  cargo run --bin trading_history                  Show today's summary + recent trades");
    println!("  cargo run --bin trading_history summary          Show today's summary");
    println!("  cargo run --bin trading_history recent [N]       Show last N trades (default: 20)");
    println!("  cargo run --bin trading_history stats            Show all-time statistics");
    println!("  cargo run --bin trading_history opportunities [N] Show last N arb opportunities");
    println!("  cargo run --bin trading_history daily [N]        Show daily summaries (last N days)");
    println!("  cargo run --bin trading_history export           Export all trades to CSV");
    println!("  cargo run --bin trading_history help             Show this help");
    println!();
}

fn print_all_time_stats(db: &TradingDatabase) -> Result<()> {
    let conn = db.conn();
    
    println!("\n All-Time Trading Statistics");
    println!("{:=<80}", "");
    
    // Total trades
    let total_trades: i64 = conn.query_row(
        "SELECT COUNT(*) FROM trades",
        [],
        |row: &rusqlite::Row| row.get(0),
    )?;
    
    // Total volume
    let total_volume: f64 = conn.query_row(
        "SELECT COALESCE(SUM(cost), 0) FROM trades",
        [],
        |row: &rusqlite::Row| row.get(0),
    )?;
    
    // Total fees
    let total_fees: f64 = conn.query_row(
        "SELECT COALESCE(SUM(fees), 0) FROM trades",
        [],
        |row: &rusqlite::Row| row.get(0),
    )?;
    
    // Total opportunities detected
    let total_opps: i64 = conn.query_row(
        "SELECT COUNT(*) FROM arbitrage_opportunities",
        [],
        |row: &rusqlite::Row| row.get(0),
    )?;
    
    // Total opportunities executed
    let executed_opps: i64 = conn.query_row(
        "SELECT COUNT(*) FROM arbitrage_opportunities WHERE executed = 1",
        [],
        |row: &rusqlite::Row| row.get(0),
    )?;
    
    // Average execution time
    let avg_exec_time: Option<f64> = conn.query_row(
        "SELECT AVG(execution_time_us) FROM trades WHERE execution_time_us IS NOT NULL",
        [],
        |row: &rusqlite::Row| row.get(0),
    ).ok();
    
    // Best trade profit
    let best_profit: Option<i64> = conn.query_row(
        "SELECT MAX(profit_cents) FROM trades WHERE profit_cents IS NOT NULL",
        [],
        |row: &rusqlite::Row| row.get(0),
    ).ok();
    
    println!("  Total Trades Executed:      {}", total_trades);
    println!("  Total Trading Volume:       ${:.2}", total_volume);
    println!("  Total Fees Paid:            ${:.2}", total_fees);
    println!("  Net Volume (after fees):    ${:.2}", total_volume - total_fees);
    println!("\n  Opportunities Detected:     {}", total_opps);
    println!("  Opportunities Executed:     {}", executed_opps);
    if total_opps > 0 {
        let success_rate = (executed_opps as f64 / total_opps as f64) * 100.0;
        println!("  Execution Success Rate:     {:.1}%", success_rate);
    }
    
    if let Some(avg_time) = avg_exec_time {
        println!("\n  Average Execution Time:     {:.0} µs ({:.2} ms)", avg_time, avg_time / 1000.0);
    }
    
    if let Some(best) = best_profit {
        println!("  Best Trade Profit:          {} cents", best);
    }
    
    // Platform breakdown
    println!("\n  Platform Breakdown:");
    let mut stmt = conn.prepare(
        "SELECT platform, COUNT(*), SUM(cost), SUM(fees) 
         FROM trades 
         GROUP BY platform"
    )?;
    
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let platform: String = row.get::<_, String>(0)?;
        let count: i64 = row.get(1)?;
        let volume: f64 = row.get(2)?;
        let fees: f64 = row.get(3)?;
        println!("    {}: {} trades, ${:.2} volume, ${:.2} fees", 
                 platform.to_uppercase(), count, volume, fees);
    }
    
    println!();
    
    Ok(())
}

fn print_recent_opportunities(db: &TradingDatabase, limit: usize) -> Result<()> {
    let conn = db.conn();
    
    println!("\n Recent Arbitrage Opportunities (Last {}):", limit);
    println!("{:-<120}", "");
    
    let mut stmt = conn.prepare(
        "SELECT timestamp, description, arb_type, yes_price, no_price, 
                profit_cents, total_cost, executed
         FROM arbitrage_opportunities 
         ORDER BY timestamp DESC 
         LIMIT ?1"
    )?;
    
    let mut rows = stmt.query(params![limit])?;
    let mut count = 0;
    
    while let Some(row) = rows.next()? {
        let timestamp: String = row.get::<_, String>(0)?;
        let description: String = row.get::<_, String>(1)?;
        let arb_type: String = row.get::<_, String>(2)?;
        let yes_price: u16 = row.get::<_, u16>(3)?;
        let no_price: u16 = row.get::<_, u16>(4)?;
        let profit_cents: i16 = row.get::<_, i16>(5)?;
        let total_cost: u16 = row.get::<_, u16>(6)?;
        let executed: bool = row.get::<_, bool>(7)?;
        
        let status = if executed { " EXECUTED" } else { "MISSED" };
        
        println!(
            "{} | {} | {} | y={}¢ n={}¢ cost={}¢ profit={}¢ | {}",
            &timestamp[11..19],  // Time only
            status,
            arb_type,
            yes_price,
            no_price,
            total_cost,
            profit_cents,
            &description[..50.min(description.len())],
        );
        count += 1;
    }
    
    if count == 0 {
        println!("No arbitrage opportunities found.");
    }
    
    println!();
    Ok(())
}

fn print_daily_summary(db: &TradingDatabase, days: usize) -> Result<()> {
    let conn = db.conn();
    
    println!("\n Daily Summary (Last {} Days):", days);
    println!("{:-<120}", "");
    println!("{:<12} {:>8} {:>12} {:>12} {:>12} {:>10} {:>10} {:>8}", 
             "Date", "Trades", "Volume", "P&L", "Fees", "Opps", "Exec", "Rate");
    println!("{:-<120}", "");
    
    let mut stmt = conn.prepare(
        "SELECT date, total_trades, total_volume, realized_pnl, total_fees,
                opportunities_detected, opportunities_executed
         FROM daily_summary 
         ORDER BY date DESC 
         LIMIT ?1"
    )?;
    
    let mut rows = stmt.query(params![days])?;
    
    while let Some(row) = rows.next()? {
        let date: String = row.get::<_, String>(0)?;
        let trades: i64 = row.get(1)?;
        let volume: f64 = row.get(2)?;
        let pnl: f64 = row.get(3)?;
        let fees: f64 = row.get(4)?;
        let opps: i64 = row.get(5)?;
        let exec: i64 = row.get(6)?;
        
        let rate = if opps > 0 {
            format!("{:.1}%", (exec as f64 / opps as f64) * 100.0)
        } else {
            "-".to_string()
        };
        
        println!("{:<12} {:>8} ${:>10.2} ${:>10.2} ${:>10.2} {:>10} {:>10} {:>8}", 
                 date, trades, volume, pnl, fees, opps, exec, rate);
    }
    
    println!();
    Ok(())
}

fn export_to_csv(db: &TradingDatabase) -> Result<()> {
    use std::fs::File;
    use std::io::Write;
    
    let conn = db.conn();
    let filename = format!("trades_export_{}.csv", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    
    let mut file = File::create(&filename)?;
    
    // Write CSV header
    writeln!(file, "timestamp,market_id,description,platform,side,contracts,price,cost,fees,order_id,arb_type,profit_cents")?;
    
    // Write all trades
    let mut stmt = conn.prepare(
        "SELECT timestamp, market_id, description, platform, side,
                contracts, price, cost, fees, order_id, arb_type, profit_cents
         FROM trades 
         ORDER BY timestamp"
    )?;
    
    let mut rows = stmt.query([])?;
    let mut count = 0;
    
    while let Some(row) = rows.next()? {
        let timestamp: String = row.get::<_, String>(0)?;
        let market_id: String = row.get::<_, String>(1)?;
        let description: String = row.get::<_, String>(2)?;
        let platform: String = row.get::<_, String>(3)?;
        let side: String = row.get::<_, String>(4)?;
        let contracts: f64 = row.get::<_, f64>(5)?;
        let price: f64 = row.get::<_, f64>(6)?;
        let cost: f64 = row.get::<_, f64>(7)?;
        let fees: f64 = row.get::<_, f64>(8)?;
        let order_id: String = row.get::<_, String>(9)?;
        let arb_type: Option<String> = row.get::<_, Option<String>>(10)?;
        let profit_cents: Option<i64> = row.get::<_, Option<i64>>(11)?;
        
        writeln!(
            file,
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            timestamp,
            market_id,
            description.replace(",", ";"),
            platform,
            side,
            contracts,
            price,
            cost,
            fees,
            order_id,
            arb_type.unwrap_or_default(),
            profit_cents.map(|p: i64| p.to_string()).unwrap_or_default(),
        )?;
        count += 1;
    }
    
    println!("\n Exported {} trades to: {}", count, filename);
    println!();
    
    Ok(())
}
