# Salimo Chat Application

## Overview
Salimo Chat is a feature-rich chat application designed to provide real-time communication capabilities.

## Features
- Real-time messaging
- Multimedia sharing
- Group chat functionality
- User authentication
- Push notifications

## Setup
1. **Clone the repository**:
   ```bash
   git clone https://github.com/haiderzaxo49-droid/salimo.git
   cd salimo
   ```
2. **Install dependencies**:
   ```bash
   npm install
   ```
3. **Environment Configuration**:
   - Create a `.env` file and set up your environment variables including Firebase credentials.

## API Endpoints
- **Login**
  - `POST /api/login`
  - Description: Authenticate users and return a token.
- **Send Message**
  - `POST /api/message`
  - Description: Send a message to a chat room.
- **Get Messages**
  - `GET /api/messages`
  - Description: Retrieve messages from a specific chat room.

## Firebase Configuration
1. Go to [Firebase Console](https://console.firebase.google.com/).
2. Create a new project.
3. Add your web app to the project.
4. Copy the Firebase configuration object from the Firebase project settings.
5. Paste the configuration into your `.env` file as follows:
   ```
   FIREBASE_API_KEY=your_api_key
   FIREBASE_AUTH_DOMAIN=your_project_id.firebaseapp.com
   FIREBASE_DATABASE_URL=https://your_project_id.firebaseio.com
   FIREBASE_PROJECT_ID=your_project_id
   FIREBASE_STORAGE_BUCKET=your_project_id.appspot.com
   FIREBASE_MESSAGING_SENDER_ID=your_messaging_sender_id
   FIREBASE_APP_ID=your_app_id
   ```

## Conclusion
With this setup, you are ready to run the Salimo Chat application. Enjoy chatting!