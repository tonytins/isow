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
        let cal = Calendar(identifier: .iso8601)
        let date = Date()
        let week = cal.component(.weekOfYear, from: date)
        let year = cal.component(.year, from: date)
        let day = cal.component(.day, from: date)
        let isoCal = "\(year)-W\(week)-\(String(format: "%02d", day))"

        print(isoCal)
    }
}
