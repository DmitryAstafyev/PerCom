package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func setupBackend() *Backend {
	backend := &Backend{
		Posts: make(map[string]Post),
		Mux:   http.ServeMux{},
	}
	backend.RegisterHandlers()
	return backend
}

func TestCreatePost(t *testing.T) {
	backend := setupBackend()
	post := Post{Author: "Alice", Date: time.Now(), Content: "Hello"}
	body, _ := json.Marshal(post)
	req := httptest.NewRequest("POST", "/posts", bytes.NewReader(body))
	w := httptest.NewRecorder()
	backend.CreatePost(w, req)
	if w.Code != http.StatusCreated {
		t.Errorf("expected status %d, got %d", http.StatusCreated, w.Code)
	}
	var created Post
	json.NewDecoder(w.Body).Decode(&created)
	if created.Author != post.Author || created.Content != post.Content {
		t.Errorf("unexpected post data: %+v", created)
	}
}

func TestGetPosts(t *testing.T) {
	backend := setupBackend()
	// Add a post
	backend.Posts["1"] = Post{ID: "1", Author: "Bob", Date: time.Now(), Content: "Hi"}
	req := httptest.NewRequest("GET", "/posts", nil)
	w := httptest.NewRecorder()
	backend.GetPosts(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}
	var posts []Post
	json.NewDecoder(w.Body).Decode(&posts)
	if len(posts) != 1 || posts[0].Author != "Bob" {
		t.Errorf("unexpected posts: %+v", posts)
	}
}

func TestGetPostById(t *testing.T) {
	backend := setupBackend()
	backend.Posts["42"] = Post{ID: "42", Author: "Carol", Date: time.Now(), Content: "Test"}
	req := httptest.NewRequest("GET", "/posts/42", nil)
	req.SetPathValue("post_id", "42")
	w := httptest.NewRecorder()
	backend.GetPostById(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}
	var post Post
	json.NewDecoder(w.Body).Decode(&post)
	if post.ID != "42" || post.Author != "Carol" {
		t.Errorf("unexpected post: %+v", post)
	}
}

func TestUpdatePostById(t *testing.T) {
	backend := setupBackend()
	backend.Posts["99"] = Post{ID: "99", Author: "Dan", Date: time.Now(), Content: "Old"}
	updated := Post{Author: "Dan", Date: time.Now(), Content: "New"}
	body, _ := json.Marshal(updated)
	req := httptest.NewRequest("PUT", "/posts/99", bytes.NewReader(body))
	req.SetPathValue("post_id", "99")
	w := httptest.NewRecorder()
	backend.UpdatePostById(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}
	var post Post
	json.NewDecoder(w.Body).Decode(&post)
	if post.Content != "New" {
		t.Errorf("expected updated content, got %+v", post)
	}
}

func TestDeletePostById(t *testing.T) {
	backend := setupBackend()
	backend.Posts["7"] = Post{ID: "7", Author: "Eve", Date: time.Now(), Content: "Bye"}
	req := httptest.NewRequest("DELETE", "/posts/7", nil)
	req.SetPathValue("post_id", "7")
	w := httptest.NewRecorder()
	backend.DeletePostById(w, req)
	if w.Code != http.StatusNoContent {
		t.Errorf("expected status %d, got %d", http.StatusNoContent, w.Code)
	}
	if _, exists := backend.Posts["7"]; exists {
		t.Errorf("post was not deleted")
	}
}

func TestGetPostById_NotFound(t *testing.T) {
	backend := setupBackend()
	req := httptest.NewRequest("GET", "/posts/404", nil)
	req.SetPathValue("post_id", "404")
	w := httptest.NewRecorder()
	backend.GetPostById(w, req)
	if w.Code != http.StatusNotFound {
		t.Errorf("expected status %d, got %d", http.StatusNotFound, w.Code)
	}
}

func TestUpdatePostById_NotFound(t *testing.T) {
	backend := setupBackend()
	updated := Post{Author: "Ghost", Date: time.Now(), Content: "Nothing"}
	body, _ := json.Marshal(updated)
	req := httptest.NewRequest("PUT", "/posts/404", bytes.NewReader(body))
	req.SetPathValue("post_id", "404")
	w := httptest.NewRecorder()
	backend.UpdatePostById(w, req)
	if w.Code != http.StatusNotFound {
		t.Errorf("expected status %d, got %d", http.StatusNotFound, w.Code)
	}
}

func TestDeletePostById_NotFound(t *testing.T) {
	backend := setupBackend()
	req := httptest.NewRequest("DELETE", "/posts/404", nil)
	req.SetPathValue("post_id", "404")
	w := httptest.NewRecorder()
	backend.DeletePostById(w, req)
	if w.Code != http.StatusNotFound {
		t.Errorf("expected status %d, got %d", http.StatusNotFound, w.Code)
	}
}

func TestCreatePost_InvalidJSON(t *testing.T) {
	backend := setupBackend()
	body := []byte("{invalid json}")
	req := httptest.NewRequest("POST", "/posts", bytes.NewReader(body))
	w := httptest.NewRecorder()
	backend.CreatePost(w, req)
	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status %d, got %d", http.StatusBadRequest, w.Code)
	}
}

func TestUpdatePostById_InvalidJSON(t *testing.T) {
	backend := setupBackend()
	body := []byte("{invalid json}")
	req := httptest.NewRequest("PUT", "/posts/1", bytes.NewReader(body))
	req.SetPathValue("post_id", "1")
	w := httptest.NewRecorder()
	backend.UpdatePostById(w, req)
	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status %d, got %d", http.StatusBadRequest, w.Code)
	}
}

// Test for JSON encoding error in GetPosts

// errorWriter simulates a ResponseWriter that always fails to write
type errorWriter struct {
	http.ResponseWriter
}

func (ew *errorWriter) Write(p []byte) (int, error) {
	return 0, fmt.Errorf("forced error")
}

type BadPost struct{}

func (b BadPost) MarshalJSON() ([]byte, error) {
	return nil, fmt.Errorf("forced error")
}

func TestGetPosts_EncodingError(t *testing.T) {
	backend := setupBackend()
	backend.Posts["1"] = Post{ID: "1", Author: "Bob", Date: time.Now(), Content: "Hi"}
	rr := httptest.NewRecorder()
	ew := &errorWriter{rr}
	req := httptest.NewRequest("GET", "/posts", nil)
	backend.GetPosts(ew, req)
	if rr.Code != http.StatusInternalServerError {
		t.Errorf("expected status %d, got %d", http.StatusInternalServerError, rr.Code)
	}
}
