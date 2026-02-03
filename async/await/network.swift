import Foundation

class APIClient {
    private let session = URLSession.shared
    private let baseURL = "https://api.example.com/v1"
    
    // 1. The "Interceptor" Logic: Injects headers and logs
    private func prepareRequest(path: String, method: String, body: Data? = nil) -> URLRequest {
        let url = URL(string: baseURL + path)!
        var request = URLRequest(url: url)
        request.httpMethod = method
        request.httpBody = body
        
        // GLOBAL HEADERS (Replaces your old 'setDefaultHeader')
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue("Bearer \(AuthManager.getToken())", forHTTPHeaderField: "Authorization")
        
        // GLOBAL LOGGING
        print("🚀 [NETWORK] \(method) to \(url.absoluteString)")
        return request
    }

    // 2. The Execution Logic
    func sendRequest<T: Decodable>(path: String, method: String = "GET", body: Data? = nil) async throws -> T {
        let request = prepareRequest(path: path, method: method, body: body)
        
        let (data, response) = try await session.data(for: request)
        
        guard let httpResponse = response as? HTTPURLResponse else { throw URLError(.badServerResponse) }
        
        // GLOBAL ERROR HANDLING (e.g., handle 401 Unauthorized)
        if httpResponse.statusCode == 401 {
            print("❌ Token expired. Redirecting to login...")
            // Logic to refresh token or logout
        }
        
        print("✅ [RESPONSE] Status: \(httpResponse.statusCode)")
        return try JSONDecoder().decode(T.self, from: data)
    }
}

Task {
    do {
        let user = try await NetworkManager.shared.fetchUser()
        print("Loaded user: \(user.name)")
    } catch {
        print("Request failed: \(error.localizedDescription)")
    }
}

// 1. Define your data model
struct User: Codable {
    let id: Int
    let name: String
}

// 2. Create a simple Network Manager
class NetworkManager {
    static let shared = NetworkManager()
    
    func fetchUser() async throws -> User {
        let url = URL(string: "https://api.example.com/user/1")!
        
        // Use the modern async data fetch method
        let (data, response) = try await URLSession.shared.data(from: url)
        
        // Ensure the server responded with a success code
        guard let httpResponse = response as? HTTPURLResponse, 
              (200...299).contains(httpResponse.statusCode) else {
            throw URLError(.badServerResponse)
        }
        
        // Automatically convert JSON data into your Swift object
        return try JSONDecoder().decode(User.self, from: data)
    }
}

func postUserData(name: String, email: String) async throws {
    let url = URL(string: "https://api.example.com/v1/users")!
    var request = URLRequest(url: url)
    
    // 1. Set the HTTP Method
    request.httpMethod = "POST"
    
    // 2. Add Headers (Replacing [self setDefaultHeader:value:])
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    request.setValue("Bearer YOUR_TOKEN_HERE", forHTTPHeaderField: "Authorization")
    
    // 3. Create and Attach POST Body
    let body: [String: String] = ["name": name, "email": email]
    request.httpBody = try? JSONSerialization.data(withJSONObject: body)
    
    // 4. Execute the request
    let (data, response) = try await URLSession.shared.data(for: request)
    
    // Handle response...
}
