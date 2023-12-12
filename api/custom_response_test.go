package api

import (
	"fmt"
	"net/http"
	"testing"

	"github.com/jiaozifs/jiaozifs/models"

	"go.uber.org/mock/gomock"
)

func TestJiaozifsResponse(t *testing.T) {
	t.Run("not found", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusNotFound)
		jzResp.NotFound()
	})

	t.Run("not found", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusUnauthorized)
		jzResp.Unauthorized()
	})

	t.Run("ok", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusOK)
		jzResp.OK()
	})

	t.Run("bad request", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusBadRequest)
		resp.EXPECT().Write([]byte("bad request"))
		jzResp.BadRequest("bad request")
	})

	t.Run("code", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusCreated)
		jzResp.Code(http.StatusCreated)
	})
	t.Run("error", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusInternalServerError)
		resp.EXPECT().Write([]byte("mock"))
		jzResp.Error(fmt.Errorf("mock"))
	})

	t.Run("error not found", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusNotFound)
		jzResp.Error(fmt.Errorf("mock %w", models.ErrNotFound))
	})

	t.Run("string", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusOK)
		resp.EXPECT().Header().DoAndReturn(func() http.Header {
			return make(http.Header)
		})
		resp.EXPECT().Write([]byte("test"))
		jzResp.String("test")
	})

	t.Run("string with code", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusCreated)
		resp.EXPECT().Header().DoAndReturn(func() http.Header {
			return make(http.Header)
		})
		resp.EXPECT().Write([]byte("test"))
		jzResp.String("test", http.StatusCreated)
	})

	t.Run("json", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusOK)
		resp.EXPECT().Header().DoAndReturn(func() http.Header {
			return make(http.Header)
		})

		resp.EXPECT().Write([]byte("{\"Name\":\"aa\"}\n"))
		jzResp.JSON(struct {
			Name string
		}{Name: "aa"})
	})
	t.Run("json with code", func(t *testing.T) {
		ctrl := gomock.NewController(t)
		resp := NewMockResponseWriter(ctrl)
		jzResp := JiaozifsResponse{resp}

		resp.EXPECT().WriteHeader(http.StatusCreated)
		resp.EXPECT().Header().DoAndReturn(func() http.Header {
			return make(http.Header)
		})

		resp.EXPECT().Write([]byte("{\"Name\":\"aa\"}\n"))
		jzResp.JSON(struct {
			Name string
		}{Name: "aa"}, http.StatusCreated)
	})

}
