// The Swift Programming Language
// https://docs.swift.org/swift-book
//
// Swift Argument Parser
// https://swiftpackageindex.com/apple/swift-argument-parser/documentation

import ArgumentParser
import Foundation

@main
struct IsoWeek: ParsableCommand {
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

        let isoCal = String(format: "%d-W%02d-%02d", year, week, day)

        print(isoCal)
    }
}
