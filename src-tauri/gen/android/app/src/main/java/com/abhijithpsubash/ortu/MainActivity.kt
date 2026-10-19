package com.abhijithpsubash.ortu

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    handleIntent(intent)
  }

  override fun onNewIntent(intent: Intent) {
    super.onNewIntent(intent)
    setIntent(intent)
    handleIntent(intent)
  }

  private fun handleIntent(intent: Intent?) {
    if (intent == null) return
    val action = intent.action
    val type = intent.type

    if (Intent.ACTION_SEND == action && type != null) {
      if ("text/plain" == type) {
        val sharedText = intent.getStringExtra(Intent.EXTRA_TEXT)
        if (sharedText != null) {
          triggerRustShareText(sharedText)
        }
      } else if (type.startsWith("image/")) {
        val imageUri = intent.getParcelableExtra<Uri>(Intent.EXTRA_STREAM)
        if (imageUri != null) {
          triggerRustShareUri(imageUri.toString())
        }
      }
    } else if (Intent.ACTION_VIEW == action) {
      val data = intent.data
      if (data != null && "ortu" == data.scheme) {
        triggerRustDeepLink(data.host ?: "")
      }
    }
  }

  private fun triggerRustShareText(text: String) {
    // Pass shared text to Tauri webview via JS evaluation or event broadcast
  }

  private fun triggerRustShareUri(uriString: String) {
    // Pass shared URI to Tauri webview
  }

  private fun triggerRustDeepLink(action: String) {
    // Trigger popup or paste target for deep link action
  }
}
