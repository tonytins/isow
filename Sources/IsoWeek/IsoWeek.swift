import ArgumentParser
import Foundation

@main
struct IsoWeek: ParsableCommand {
    @Flag(name: .shortAndLong, help: "Switch to UTC time zone.")
    var utc: Bool = false

    @Flag(name: .shortAndLong, help: "Show the complete date.")
    var full: Bool = false

    mutating func run() throws {
        // Initialize date and calendar
        let date = Date()
        var cal = Calendar(identifier: .iso8601)

        // Grab calendar data
        let week = cal.component(.weekOfYear, from: date)
        let year = cal.component(.year, from: date)
        let day = cal.component(.day, from: date)

        // ISO rules
        cal.firstWeekday = 2
        cal.minimumDaysInFirstWeek = 4

        if utc {
            let timeZone = TimeZone(abbreviation: "UTC")

            // On macOS 13 or newer and other platforms, fallback to GMT.
            // If the user is on an older version of macOS, fallback to their current time zone.
            if #available(macOS 13, *) {
                cal.timeZone = timeZone ?? .gmt
            } else {
                cal.timeZone = timeZone ?? .autoupdatingCurrent
            }
        }

        switch full {
        case true:
            let fullDate = String(format: "%d-W%02d-%02d", year, week, day)
            print(fullDate)
        default:
            print(String(format: "W%02d", week))
        }
    }
}
