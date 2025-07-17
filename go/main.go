package main

import (
	"encoding/json"
	"net/http"
	"sync"
	"time"

	"github.com/google/uuid"
)

type Post struct {
	ID      string    `json:"id"`
	Author  string    `json:"author"`
	Date    time.Time `json:"date"`
	Content string    `json:"content"`
}

type Backend struct {
	Posts map[string]Post
	Lock  sync.Mutex
	Mux   http.ServeMux
}

func (b *Backend) GetPosts(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	b.Lock.Lock()
	defer b.Lock.Unlock()
	postsList := make([]Post, 0, len(b.Posts))
	for _, post := range b.Posts {
		postsList = append(postsList, post)
	}
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(postsList); err != nil {
		http.Error(w, "Failed to encode JSON", http.StatusInternalServerError)
		return
	}
}

func (b *Backend) CreatePost(w http.ResponseWriter, r *http.Request) {
	var post Post
	if err := json.NewDecoder(r.Body).Decode(&post); err != nil {
		http.Error(w, "Failed to decode JSON", http.StatusBadRequest)
		return
	}
	post.ID = uuid.New().String()
	b.Lock.Lock()
	defer b.Lock.Unlock()
	b.Posts[post.ID] = post
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(post)
}

func (b *Backend) GetPostById(w http.ResponseWriter, r *http.Request) {
	postID := r.PathValue("post_id")
	b.Lock.Lock()
	defer b.Lock.Unlock()
	post, exists := b.Posts[postID]
	if !exists {
		http.Error(w, "Post not found", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(post)
}

func (b *Backend) UpdatePostById(w http.ResponseWriter, r *http.Request) {
	postID := r.PathValue("post_id")
	var updatedPost Post
	if err := json.NewDecoder(r.Body).Decode(&updatedPost); err != nil {
		http.Error(w, "Failed to decode JSON", http.StatusBadRequest)
		return
	}
	updatedPost.ID = postID
	b.Lock.Lock()
	defer b.Lock.Unlock()
	if _, ok := b.Posts[postID]; !ok {
		http.Error(w, "Post not found", http.StatusNotFound)
		return
	}
	b.Posts[postID] = updatedPost
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(updatedPost)
}

func (b *Backend) DeletePostById(w http.ResponseWriter, r *http.Request) {
	postID := r.PathValue("post_id")
	b.Lock.Lock()
	defer b.Lock.Unlock()
	if _, ok := b.Posts[postID]; !ok {
		http.Error(w, "Post not found", http.StatusNotFound)
		return
	}
	delete(b.Posts, postID)
	w.WriteHeader(http.StatusNoContent)
}

func (b *Backend) RegisterHandlers() {
	b.Mux.HandleFunc("GET /posts", b.GetPosts)
	b.Mux.HandleFunc("POST /posts", b.CreatePost)
	b.Mux.HandleFunc("GET /posts/{post_id}", b.GetPostById)
	b.Mux.HandleFunc("PUT /posts/{post_id}", b.UpdatePostById)
	b.Mux.HandleFunc("DELETE /posts/{post_id}", b.DeletePostById)
}

func main() {
	backend := Backend{
		Posts: make(map[string]Post),
		Mux:   *http.NewServeMux(),
	}
	backend.RegisterHandlers()
	http.ListenAndServe(":8080", &backend.Mux)
}
