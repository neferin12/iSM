#include <napi.h>
extern "C" {
#include <ism/algorithm.h>
#include <ism/io.h>
}

using namespace Napi;

Napi::Value Method(const Napi::CallbackInfo& info) {
  Napi::Env env = info.Env();
  if (info.Length() < 2) {
        Napi::TypeError::New(env, "Two arguments expected").ThrowAsJavaScriptException();
        return env.Null();
  }
  if(!info[0].IsString() || !info[1].IsString()) {
        Napi::TypeError::New(env, "Two string arguments expected").ThrowAsJavaScriptException();
        return env.Null();
  }

  GArray *w_seminars = getSeminars(info[0].ToString().Utf8Value().c_str(), 'w');
  return Napi::String::New(env, "lol");
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports.Set(Napi::String::New(env, "TsismNapi"),
              Napi::Function::New(env, Method));
  return exports;
}

NODE_API_MODULE(addon, Init)
