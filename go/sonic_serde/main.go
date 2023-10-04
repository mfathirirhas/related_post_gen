package main

import (
	"fmt"
	"log"
	"os"
	"time"

	"github.com/bytedance/sonic"
)

type Post struct {
	ID    string   `json:"_id"`
	Title string   `json:"title"`
	Tags  []string `json:"tags"`
}

type PostWithSharedTags struct {
	Post       int
	SharedTags int
}

type RelatedPosts struct {
	ID      string   `json:"_id"`
	Tags    []string `json:"tags"`
	Related []*Post  `json:"related"`
}

func main() {
	startIOSerde := time.Now()
	file, err := os.Open("../posts.json")
	if err != nil {
		log.Panicln(err)
	}

	startSerde := time.Now()
	var posts []Post
	err = sonic.ConfigFastest.NewDecoder(file).Decode(&posts)
	if err != nil {
		log.Panicln(err)
	}

	start := time.Now()

	// assumes that there are less than 100 tags
	tagMap := make(map[string][]int, 100)

	for i, post := range posts {
		for _, tag := range post.Tags {
			tagMap[tag] = append(tagMap[tag], i)
		}
	}

	allRelatedPosts := make([]RelatedPosts, 0, len(posts))
	taggedPostCount := make([]int, len(posts))

	for i := range posts {
		// optimized to a memset
		for j := range taggedPostCount {
			taggedPostCount[j] = 0
		}

		for _, tag := range posts[i].Tags {
			for _, otherPostIdx := range tagMap[tag] {
				taggedPostCount[otherPostIdx]++
			}
		}

		taggedPostCount[i] = 0 // Don't count self

		top5 := [5]PostWithSharedTags{}
		minTags := 0 // Updated initialization

		for j, count := range taggedPostCount {
			if count > minTags {
				// Find the position to insert
				pos := 4
				for pos >= 0 && top5[pos].SharedTags < count {
					pos--
				}
				pos++

				// Shift and insert
				if pos < 4 {
					copy(top5[pos+1:], top5[pos:4])
				}
				if pos <= 4 {
					top5[pos] = PostWithSharedTags{Post: j, SharedTags: count}
					minTags = top5[4].SharedTags
				}
			}
		}

		// Convert indexes back to Post pointers
		topPosts := make([]*Post, 0, 5)
		for _, t := range top5 {
			if t.SharedTags > 0 {
				topPosts = append(topPosts, &posts[t.Post])
			}
		}

		allRelatedPosts = append(allRelatedPosts, RelatedPosts{
			ID:      posts[i].ID,
			Tags:    posts[i].Tags,
			Related: topPosts,
		})
	}

	end := time.Now()

	duration := end.Sub(start)
	durationSerde := end.Sub(startSerde)
	durationIOSerde := end.Sub(startIOSerde)

	serdeOverhead := durationSerde - duration
	ioOverhead := durationIOSerde - durationSerde
	ioSerdeOverhead := durationIOSerde - duration

	fmt.Println("Processing time:", duration)
	fmt.Println("Processing time with serde:", durationSerde)
	fmt.Println("Processing time with IO and serde:", durationIOSerde)

	fmt.Println("Overhead serde:", serdeOverhead)
	fmt.Println("Overhead IO:", ioOverhead)
	fmt.Println("Overhead IO & serde:", ioSerdeOverhead)

	file, err = os.Create("../related_posts_go.json")
	if err != nil {
		log.Panicln(err)
	}

	err = sonic.ConfigFastest.NewEncoder(file).Encode(allRelatedPosts)
	if err != nil {
		log.Panicln(err)
	}
}
