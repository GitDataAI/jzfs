package ipfs

import (
	"context"
	"fmt"
	"github.com/modern-go/reflect2"
	"net/url"
	"sort"
	"strings"

	iface "github.com/ipfs/kubo/core/coreiface"

	"github.com/ipfs/kubo/core/coreiface/options"

	"github.com/ipfs/boxo/path"

	"github.com/ipfs/kubo/client/rpc"
	"github.com/jiaozifs/jiaozifs/block"
)

type Walker struct {
	client *rpc.HttpApi
	mark   block.Mark
}

func NewIPFSWalker(client *rpc.HttpApi) *Walker {
	return &Walker{
		client: client,
		mark:   block.Mark{HasMore: true},
	}
}

func (s *Walker) Walk(ctx context.Context, storageURI *url.URL, op block.WalkOptions, walkFn func(e block.ObjectStoreEntry) error) error {
	const maxKeys = 1000
	prefix := strings.TrimLeft(storageURI.Path, "/")

	curPath, err := path.NewPath(prefix)
	if err != nil {
		return err
	}
	resultCh, err := s.client.Unixfs().Ls(ctx, curPath, options.Unixfs.ResolveChildren(true))
	if !reflect2.IsNil(err) {
		return err
	}

	entries, err := s.getAllEntries(resultCh, prefix)
	if err != nil {
		return err
	}

	startIndex := sort.Search(len(entries), func(i int) bool {
		return entries[i].FullKey > op.ContinuationToken && entries[i].FullKey > op.After
	})
	for i := startIndex; i < len(entries); i++ {
		err := walkFn(entries[i])
		if err != nil {
			return err
		}
		s.mark.LastKey = entries[i].FullKey
		s.mark.ContinuationToken = entries[i].FullKey
	}

	s.mark = block.Mark{
		LastKey: "",
		HasMore: false,
	}
	return nil
}
func (s *Walker) getAllEntries(ch <-chan iface.DirEntry, prefix string) ([]block.ObjectStoreEntry, error) {
	var entries []block.ObjectStoreEntry
	for record := range ch {
		if record.Type == iface.TFile {
			addr := fmt.Sprintf("ipfs://%s/%s", prefix, record.Name)
			ent := block.ObjectStoreEntry{
				FullKey:     fmt.Sprintf("%s/%s", prefix, record.Name),
				RelativeKey: record.Name,
				Address:     addr,
				Size:        int64(record.Size),
			}
			entries = append(entries, ent)
		}
	}

	sort.Slice(entries, func(i, j int) bool {
		return entries[i].FullKey < entries[j].FullKey
	})
	return entries, nil
}

func (s *Walker) Marker() block.Mark {
	return s.mark
}

func (s *Walker) GetSkippedEntries() []block.ObjectStoreEntry {
	return nil
}
