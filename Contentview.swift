import SwiftUI

struct Item: Identifiable, Codable {
    var id: Int?
    var name: String
}

class DatabaseAPI: ObservableObject {
    @Published var items: [Item] = []
    let baseURL = "https://your-backend-domain.com" // change later

    func fetchItems() async {
        guard let url = URL(string: "\(baseURL)/items") else { return }
        do {
            let (data, _) = try await URLSession.shared.data(from: url)
            let decoded = try JSONDecoder().decode([Item].self, from: data)
            DispatchQueue.main.async {
                self.items = decoded
            }
        } catch {
            print("Fetch error: \(error)")
        }
    }

    func addItem(name: String) async {
        guard let url = URL(string: "\(baseURL)/items") else { return }
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        let body = try? JSONEncoder().encode(Item(name: name))
        request.httpBody = body

        do {
            let (data, _) = try await URLSession.shared.data(for: request)
            let newItem = try JSONDecoder().decode(Item.self, from: data)
            DispatchQueue.main.async {
                self.items.append(newItem)
            }
        } catch {
            print("Add error: \(error)")
        }
    }
}

struct ContentView: View {
    @StateObject private var api = DatabaseAPI()
    @State private var newName = ""

    var body: some View {
        NavigationView {
            VStack {
                HStack {
                    TextField("Enter name", text: $newName)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                    Button("Add") {
                        Task {
                            await api.addItem(name: newName)
                            newName = ""
                        }
                    }
                }
                .padding()

                List(api.items) { item in
                    Text(item.name)
                }
            }
            .navigationTitle("Shared Items")
            .task { await api.fetchItems() }
        }
    }
}

@main
struct MultiDeviceApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
