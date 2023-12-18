#include <napi.h>
#include <glib-2.0/glib.h>
#include <glib-2.0/glib/garray.h>

extern "C" {
#include <ism/algorithm.h>
#include <ism/io.h>
#include <ism/organization.h>
}

using namespace Napi;

//struct student {
//    char *name;
//    int mimiPoints;
//    seminar *wVotes;
//    seminar *pVotes;
//    seminar wSeminar;
//    seminar pSeminar;
//};

Object convertSeminarToNapi(const Napi::Env env, const seminar *sem) {
    Object seminar_object = Object::New(env);
    seminar_object.Set(String::New(env, "id"), Number::New(env, *(sem->id)));
    seminar_object.Set(String::New(env, "name"), String::New(env, sem->name));
    seminar_object.Set(String::New(env, "size"), Number::New(env, sem->size));
    return seminar_object;
}

seminar convertNapiSeminartoStruct(const Object seminar_object) {
    seminar sem;
    sem.id = new int(seminar_object.Get("id").ToNumber().Int32Value());
    sem.name = new char[seminar_object.Get("name").ToString().Utf8Value().length() + 1];
    strcpy(sem.name, seminar_object.Get("name").ToString().Utf8Value().c_str());
    sem.size = seminar_object.Get("size").ToNumber().Int32Value();
    return sem;
}

Array convertSeminarsToJSArray(const Env env, const GArray *seminars) {
    Array seminars_array = Array::New(env, seminars->len);
    for (uint i = 0; i < seminars->len; i++) {
        auto *sem = &g_array_index(seminars, seminar, i);
        Object seminar_object = convertSeminarToNapi(env, sem);
        seminars_array.Set(i, seminar_object);
    }
    return seminars_array;
}

GArray *convertJSArrayToSeminars(const Array seminars_array) {
    GArray *seminars = g_array_new(FALSE, FALSE, sizeof(seminar));
    for (uint i = 0; i < seminars_array.Length(); i++) {
        auto seminar_object = seminars_array.Get(i).ToObject();
        seminar sem = convertNapiSeminartoStruct(seminar_object);
        g_array_append_val(seminars, sem);
    }
    return seminars;
}


Value runAlgorithm(const CallbackInfo &info) {
    Env env = info.Env();
    if (info.Length() < 3) {
        TypeError::New(env, "Three arguments expected").ThrowAsJavaScriptException();
        return env.Null();
    }
    if (!info[0].IsArray() || !info[1].IsArray()|| !info[2].IsArray()) {
        TypeError::New(env, "Array argument expected").ThrowAsJavaScriptException();
        return env.Null();
    }
    GArray * w_seminars = convertJSArrayToSeminars(info[0].As<Array>());
    printf("w_seminars: %d\n", w_seminars->len);

    GArray * p_seminars = convertJSArrayToSeminars(info[1].As<Array>());
    printf("p_seminars: %d\n", p_seminars->len);
    return String::New(env, "lol");
}

Value loadSeminars(const CallbackInfo &info) {
    Env env = info.Env();
    if (info.Length() < 1) {
        TypeError::New(env, "One argument expected").ThrowAsJavaScriptException();
        return env.Null();
    }
    if (!info[0].IsString()) {
        TypeError::New(env, "String argument expected").ThrowAsJavaScriptException();
        return env.Null();
    }
    GArray * w_seminars = getSeminars(info[0].ToString().Utf8Value().c_str(), 'W');
    GArray * p_seminars = getSeminars(info[0].ToString().Utf8Value().c_str(), 'P');

    Object seminars = Object::New(env);

    Array w_seminars_array = convertSeminarsToJSArray(env, w_seminars);
    Array p_seminars_array = convertSeminarsToJSArray(env, p_seminars);

    seminars.Set(String::New(env, "w_seminars"), w_seminars_array);
    seminars.Set(String::New(env, "p_seminars"), p_seminars_array);

    return seminars;
}

Object Init(Env env, Object exports) {
    exports.Set(String::New(env, "runAlgorithm"),
                Function::New(env, static_cast<Value(*)(const CallbackInfo&)>(&runAlgorithm), "runAlgorithm"));
    exports.Set(String::New(env, "loadSeminars"),
                Function::New(env, static_cast<Value(*)(const CallbackInfo&)>(&loadSeminars), "loadSeminars"));
    return exports;
}

NODE_API_MODULE(addon, Init)
