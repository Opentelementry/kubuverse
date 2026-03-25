import SwiftUI

// 1. The Modern Network Manager
class NetworkManager: NSObject, ObservableObject, URLSessionTaskDelegate {
    static let shared = NetworkManager()
    @Published var uploadProgress: Double = 0.0
    
    // Background session ensures uploads finish even if app is closed
    private lazy var session: URLSession = {
        let config = URLSessionConfiguration.background(withIdentifier: "com.yourapp.upload")
        config.isDiscretionary = false // Run immediately
        return URLSession(configuration: config, delegate: self, delegateQueue: .main)
    }()

    func uploadFile(fileURL: URL) {
        var request = URLRequest(url: URL(string: "https://api.example.com/upload")!)
        request.httpMethod = "POST"
        
        // GLOBAL HEADERS (Replaces your old 'setDefaultHeader')
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        request.setValue("Bearer YOUR_TOKEN", forHTTPHeaderField: "Authorization")
        
        // Background sessions REQUIRE using a file on disk, not raw Data
        let task = session.uploadTask(with: request, fromFile: fileURL)
        task.resume()
    }

    // INTERCEPTOR: Track progress in real-time
    func urlSession(_ session: URLSession, task: URLSessionTask, didSendBodyData bytesSent: Int64, totalBytesSent: Int64, totalBytesExpectedToSend: Int64) {
        self.uploadProgress = Double(totalBytesSent) / Double(totalBytesExpectedToSend)
    }
}

// 2. The Modern SwiftUI Interface
struct ContentView: View {
    @StateObject var network = NetworkManager.shared
    
    var body: some View {
        VStack(spacing: 30) {
            // Native ProgressView
            ProgressView("Uploading...", value: network.uploadProgress, total: 1.0)
                .progressViewStyle(.linear)
                .tint(.blue)
            
            Button(action: startUpload) {
                Label("Upload Image", systemImage: "arrow.up.doc")
                    .padding()
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
        }
        .padding()
    }
    
    func startUpload() {
        // Assume you have a local file URL for an image
        if let fileURL = Bundle.main.url(forResource: "example", withExtension: "jpg") {
            network.uploadFile(fileURL: fileURL)
        }
    }
}
