// This source file is part of the Swift.org open source project
// Copyright (c) 2014 - 2017 Apple Inc. and the Swift project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://swift.org/LICENSE.txt for license information
// See https://swift.org/CONTRIBUTORS.txt for the list of Swift project authors

// RUN: not %target-swift-frontend %s -typecheck
import Foundation

// A simple protocol
protocol Storable {
    var id: UUID { get }
}

// A struct that conforms to the protocol
struct Item: Storable {
    let id: UUID
    let name: String
}

// A class that acts like a "database"
class Database {
    private var storage: [UUID: Item] = [:]

    func insert(_ item: Item) {
        storage[item.id] = item
    }

    func fetch(id: UUID) -> Item? {
        return storage[id]
    }

    func allItems() -> [Item] {
        return Array(storage.values)
    }
}

// Example usage
let db = Database()
let newItem = Item(id: UUID(), name: "Test Object")

db.insert(newItem)

if let fetched = db.fetch(id: newItem.id) {
    print("Fetched item: \(fetched.name)")
}

print("All items in DB: \(db.allItems().map { $0.name })")
